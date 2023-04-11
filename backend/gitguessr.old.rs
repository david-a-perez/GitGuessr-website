use std::{io::BufRead, io::Write, path::PathBuf};

use gix::{
    bstr::{BStr, ByteSlice},
    clone::PrepareFetch,
    create,
    hash::ObjectId,
    objs::{Kind, TreeRefIter},
    object::tree::diff::{change::Event, Action},
    open,
    traverse::tree::{recorder::Entry, Recorder},
    Commit, Id, Repository,
};
use rand::seq::{IteratorRandom, SliceRandom};
use thiserror::Error;

use prodash::Root;

#[derive(Error, Debug)]
enum GitGuessrError {
    #[error("git-oxide clone error: {0}")]
    GitOxideClone(#[from] gix::clone::Error),

    #[error("git-oxide clone fetch error: {0}")]
    GitOxideCloneFetch(#[from] gix::clone::fetch::Error),

    #[error("git-oxide open error: {0}")]
    GitOxideOpen(#[from] gix::open::Error),

    #[error("git-oxide find existing reference error: {0}")]
    GitOxideFindExistingReference(#[from] gix::reference::find::existing::Error),

    #[error("git-oxide find existing reference error: {0}")]
    GitOxideTraverseCommitAncestors(#[from] gix::traverse::commit::ancestors::Error),

    #[error("git-oxide Odb find existing error: {0}")]
    GitOxideTraverseOdbFindExisting(
        #[from] gix::odb::find::existing::Error<gix::odb::store::find::Error>,
    ),

    #[error("git-oxide Odb store find error: {0}")]
    GitOxideTraverseOdbStoreFind(#[from] gix::odb::store::find::Error),

    #[error("git-oxide object conversion error: {0}")]
    GitOxideObjectConversion(#[from] gix::object::conversion::Error),

    #[error("git-oxide peel head to commit error: {0}")]
    GitOxidePeelHeadToCommit(#[from] gix::head::peel::to_commit::Error),

    #[error("git-oxide object commit error: {0}")]
    GitOxideObjectCommit(#[from] gix::object::commit::Error),

    #[error("git-oxide object decode error: {0}")]
    GitOxideObjectDecode(#[from] gix::objs::decode::Error),

    #[error("git-oxide tree traversal error: {0}")]
    GitOxideTreeTraversal(#[from] gix::traverse::tree::breadthfirst::Error),

    #[error("git-oxide object try into error: {0}")]
    GitOxideObjectTryInto(#[from] gix::object::try_into::Error),

    #[error("git-oxide object peel to kind error: {0}")]
    GitOxideObjecPeelToKind(#[from] gix::object::peel::to_kind::Error),

    #[error("git-oxide object peel to kind error: {0}")]
    GitOxideTreeDiffForEach(#[from] gix::object::tree::diff::for_each::Error),

    #[error("Non UTF-8 file contents in '{path}': {error}")]
    NonUtf8Text {
        path: String,
        error: std::str::Utf8Error,
    },

    #[error("Could not create TempDir: {0}")]
    CreateTempDir(std::io::Error),

    #[error("Could not read line from stdin: {0}")]
    ReadStdinLine(std::io::Error),

    #[error("Could not init interupt handler: {0}")]
    GitOxideInitInteruptHandler(std::io::Error),

    #[error("GitOxide could not find id: {0}")]
    GitOxideUnknownId(String),
}

type Result<T> = std::result::Result<T, GitGuessrError>;

fn get_all_file_entries(repo: &Repository) -> Result<Vec<Entry>> {
    let head = repo.head()?;

    let mut recorder = Recorder::default();

    let id = head.id().unwrap();
    let object = repo.try_find_object(id)?;
    let object =
        object.ok_or_else(|| GitGuessrError::GitOxideUnknownId(id.to_hex().to_string()))?;
    let tree = object.try_to_commit_ref()?.tree();
    let tree = repo.try_find_object(tree)?.unwrap();
    let tree = tree.try_into_tree()?;
    tree.traverse().breadthfirst(&mut recorder)?;

    Ok(recorder.records)
}

fn get_random_entries<'a, V, C>(entries: &'a V, amount: usize) -> C
where
    V: AsRef<[Entry]>,
    C: FromIterator<&'a Entry>,
{
    entries
        .as_ref()
        .choose_multiple(&mut rand::thread_rng(), amount)
        .collect()
}

fn get_snippet_from_file<'a>(path: &BStr, text: &'a [u8], size: usize) -> Result<Vec<&'a str>> {
    let all_lines = std::str::from_utf8(text)
        .map_err(|err| GitGuessrError::NonUtf8Text {
            path: path.to_string(),
            error: err,
        })?
        .lines()
        .collect::<Vec<_>>();
    let final_result = all_lines
        .windows(size)
        .choose(&mut rand::thread_rng())
        .map(|x| x.to_vec())
        .unwrap_or(all_lines);
    Ok(final_result)
}

fn get_all_commits(repo: &Repository, tips: Vec<ObjectId>) -> Result<Vec<Commit>> {
    let mut result = Vec::new();
    for id in repo.rev_walk(tips.into_iter()).all()? {
        let id = id?;
        let object = repo.try_find_object(id)?;
        let object =
            object.ok_or_else(|| GitGuessrError::GitOxideUnknownId(id.to_hex().to_string()))?;
        result.push(object.try_into_commit()?);
    }
    Ok(result)
}

fn get_all_commits2(repo: &Repository, tips: Vec<ObjectId>) -> Result<Vec<Id>> {
    let mut result = Vec::new();
    for id in repo.rev_walk(tips.into_iter()).all()? {
        let id = id?;
        let object = repo.try_find_object(id)?;
        let object =
            object.ok_or_else(|| GitGuessrError::GitOxideUnknownId(id.to_hex().to_string()))?;
        // result.push(object.try_to_commit_ref()?);
        result.push(id);
    }
    Ok(result)
}

fn get_all_commits3(repo: &Repository, tips: Vec<ObjectId>) -> Result<Vec<Id>> {
    let mut result = Vec::new();
    for id in repo.rev_walk(tips.into_iter()).all()? {
        let id = id?;
        let object = repo.try_find_object(id)?;
        let object =
            object.ok_or_else(|| GitGuessrError::GitOxideUnknownId(id.to_hex().to_string()))?;
        let commit = object.try_into_commit()?;
        let parents: Vec<_> = commit.parent_ids().collect();
        println!("{:?}", parents);
        if let &[parent] = &parents[..] {
            let parent = repo
                .try_find_object(parent)?
                .ok_or_else(|| GitGuessrError::GitOxideUnknownId(id.to_hex().to_string()))?
                .try_into_commit()?;

            let mut count = 0;

            if let Err(GitGuessrError::GitOxideTreeDiffForEach(gix::object::tree::diff::for_each::Error(Action::Cancelled))) = parent.tree()?.changes().for_each_to_obtain_tree(&commit.tree()?, |change| -> Result<Action>{
                // match change.event {
                //     Event::Addition { entry_mode, id } => todo!(),
                //     Event::Deletion { entry_mode, id } => todo!(),
                //     Event::Modification { previous_entry_mode, previous_id, entry_mode, id } => todo!(),
                // }
                count += 1;
                
                if count > 1 {
                    Ok(Action::Cancel)
                } else {
                    Ok(Action::Continue)
                }
                
            }) {
                continue;
            }
            println!("{count}");

            if count == 1 {
                println!("{} {}", commit.id(), commit.message()?.title)
            }
            // let parent = Changes::from(Some(TreeRefIter::from_bytes(&parent.data)));
            // parent.needed_to_obtain(TreeRefIter::from_bytes(&commit.data), &mut gix::diff::tree::State::default(), |o, v| , delegate)
        }
        result.push(id);
    }
    Ok(result)
}

// const REPO_NAME: &'static str = "git2-rs";
// const REPO_URL: &'static str = "https://github.com/rust-lang/git2-rs.git";

// const REPO_NAME: &'static str = "BizHawk";
// const REPO_URL: &'static str = "https://github.com/TASEmulators/BizHawk.git";

// const REPO_NAME: &'static str = "linux";
// const REPO_URL: &'static str = "https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git";

const REPO_NAME: &'static str = "gitoxide";
const REPO_URL: &'static str = "https://github.com/Byron/gitoxide.git";

// const REPO_NAME: &'static str = "CS395";
// const REPO_URL: &'static str = "https://github.com/douglascraigschmidt/CS395.git";

const SNIPPET_LEN: usize = 30;
const NUM_ROUNDS: usize = 5;

fn main() -> Result<()> {
    let mut temp_dir = PathBuf::from(std::env::temp_dir());
    temp_dir.push("GitGuessr");
    if !temp_dir.exists() {
        std::fs::create_dir(&temp_dir).map_err(|err| GitGuessrError::CreateTempDir(err))?;
    }
    temp_dir.push(REPO_NAME);
    let repo = if !temp_dir.exists() {
        let root = prodash::tree::Root::new();
        let mut progress = root.add_child("clone");

        progress.init(None, None);

        let render_line_handle = prodash::render::line::render(
            std::io::stdout(),
            root.downgrade(),
            prodash::render::line::Options::default(),
        );

        gix::interrupt::init_handler(|| {})
            .map_err(|err| GitGuessrError::GitOxideInitInteruptHandler(err))?;

        let (repo, _outcome) = PrepareFetch::new(
            REPO_URL,
            &temp_dir,
            create::Kind::Bare,
            create::Options::default(),
            open::Options::isolated()
                .config_overrides(["user.name=gitoxide", "user.email=gitoxide@localhost"]),
        )?
        .fetch_only(progress, &gix::interrupt::IS_INTERRUPTED)?;

        render_line_handle.shutdown_and_wait();

        repo
    } else {
        open(&temp_dir)?
    };

    let head = repo.head()?;

    let commits = get_all_commits3(&repo, vec![repo.head()?.id().unwrap().into()])?;
    todo!();

    let entries = get_all_file_entries(&repo)?;
    println!("File Entries len: {}", entries.len());
    let chosen_entries: Vec<_> = get_random_entries(&entries, NUM_ROUNDS);

    for entry in chosen_entries {
        let object = repo.try_find_object(entry.oid)?.unwrap();

        // TODO: only allow files in recorder
        let blob = object.peel_to_kind(Kind::Blob).unwrap();

        let text = &blob.data;

        let snippet = get_snippet_from_file(entry.filepath.as_ref(), &text, SNIPPET_LEN)?;

        println!("---------------------");
        for line in snippet {
            println!("{line}");
        }
        println!("---------------------");

        let stdin = std::io::stdin();
        let mut handle = stdin.lock();

        let mut line = String::new();

        let mut target = PathBuf::new();

        loop {
            let root = repo
                .try_find_object(head.id().unwrap())?
                .unwrap()
                .peel_to_tree()?;

            let current_dir = if !target.as_os_str().is_empty() {
                root.lookup_entry_by_path(&target)?
                    .unwrap()
                    .object()?
                    .peel_to_tree()?
            } else {
                root
            };

            for entry in current_dir.iter() {
                let entry = entry?;
                println!("{:?} {}", entry.mode(), entry.filename())
            }

            print!("> ");
            std::io::stdout().flush().unwrap();
            line.clear();
            handle
                .read_line(&mut line)
                .map_err(|err| GitGuessrError::ReadStdinLine(err))?;

            if line
                .trim_end_matches("\n")
                .trim_end_matches("\r")
                .is_empty()
            {
                println!("Skip (q|s)? ");
                line.clear();
                handle
                    .read_line(&mut line)
                    .map_err(|err| GitGuessrError::ReadStdinLine(err))?;
                match &*line
                    .trim_end_matches("\n")
                    .trim_end_matches("\r")
                    .to_ascii_lowercase()
                {
                    "q" | "s" => break,
                    _ => continue,
                }
            }

            let line = line.trim_end_matches("\n").trim_end_matches("\r");
            if line == ".." {
                target.pop();
            }

            if let Some(selected_file) = current_dir.lookup_entry_by_path(line)? {
                target.push(selected_file.filename().to_path().unwrap());
                match selected_file.mode() {
                    gix::objs::tree::EntryMode::Blob => break,
                    _ => {}
                }
            }
        }

        println!("GitGuessr: {} Guess: {:?}", entry.filepath, target);
    }

    let start = std::time::Instant::now();
    let commits = get_all_commits2(&repo, vec![repo.head()?.id().unwrap().into()])?;
    println!(
        "Commits len: {}, duration: {} secs",
        commits.len(),
        (std::time::Instant::now() - start).as_secs_f64()
    );

    // let start = std::time::Instant::now();
    // let _ = std::hint::black_box(get_all_commits2(&repo, vec![repo.head()?.id().unwrap().into()])?);
    // println!("Commits len: {}, duration: {} secs", 0, (std::time::Instant::now() - start).as_secs_f64());

    Ok(())
}

// Big issues that are foreseen:
// Employee Single Sign On
// Automatic downloading of private repositories
//   We need a private key that can view the repository
// libgit2's implementation of git blame is *extremely* slow
//   We would need to use the git CLI for blame
// Making blame interesting and fun (most repos are led by one developer)
// CVE might not be mentioned in commit message
//   May need integration with ticket system

// Features:
// Allow for a "pathspec" besides HEAD to be used
// Allow for the number of lines to be customizable and potentially dynamic to content
// Allow for git blame to be for only part of the file
// Allow for users to download and delete repo's from the website
// Allow for users to see download progress on website
// Allow for users to create and play quizzes
// Allow for users to create and join lobbies
// Monthly/Weekly Leaderboard
// Clone with GitOxide for faster speeds
// Support filters (folders that should not be used, folders that are exclusivly used, and the same for file extensions)

// Potential features:
// Allow for scrolling and context menu (Ctrl-click) for point reduction
// Use WASM vim window
// Editor plugin
