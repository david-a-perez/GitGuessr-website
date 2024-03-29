import React from 'react'

export const Home = () => {

  const [width, setWidth] = React.useState(window.innerWidth)

  React.useEffect(() => {
    const handleResizeWindow = () => setWidth(window.innerWidth)
    // subscribe to window resize event "onComponentDidMount"
    window.addEventListener("resize", handleResizeWindow)
    return () => {
      // unsubscribe "onComponentDestroy"
      window.removeEventListener("resize", handleResizeWindow)
    }
  }, [])

  return (
    <div style={{ height: '92vh', overflowY: 'scroll' }}>
      <div style={{ height: '100vh', display: 'block' }}>
        <br />
        <h1 className='mb-5 mt-4'>Welcome to GitGuessr!</h1>
        <br />
        <div className='menu' style={{ textAlign: 'left' }}>
          <div className='container'>
            <div className='row'>
              <div className='col-1' />
              <div className='col'>
                <div className="card h-80" style={{ overflowY: 'hidden' }}>
                  <h5 className="card-header">About</h5>
                  <div className="card-body">
                    <p className="card-text">
                      GitGuessr is designed to help you learn a new codebase, or stay up to date on one you
                      already use. The goal of this project is to create a tool that is both secure enough and
                      flexible enough to be useful in industry applications. To learn more about GitGuessr or
                      to clone the repository and use it for your team, visit
                      our <a href="https://github.com/david-a-perez/GitGuessr-website" target="_blank">github page
                      </a>.
                    </p>
                  </div>
                </div>
              </div>
              <div className='col' style={{ paddingLeft: "10%" }}>
                <img
                  src='https://user-images.githubusercontent.com/46609460/231286372-f3968e6c-b5c3-4e11-a1aa-22f76541830c.png'
                  alt='logo'
                  width='300px'
                  height='300px'
                />
              </div>
              <div className='col-1' />
            </div>
          </div>
        </div>
      </div>
      <div style={{ height: '100vh', display: 'block', backgroundColor: 'lightgrey' }}>
        <br />
        <h1 className='mb-5 mt-4'>Gamemode-Obfuscated</h1>
        <br />
        <div className='container' style={{ textAlign: 'left', overflowY: '-moz-hidden-unscrollable' }}>
          <div className='row'>
            <div className='col'>
              <h4>
                In the obfuscated game mode, a player is given a random function or struct with
                obfuscated keywords. The player's goal is to guess the name of the function
                from a list of potential choices. The level of keyword abstraction can be set to
                make the game harder or easier. This game mode exposes developers to new functions
                and pieces of code that they may be familiar with but have never seen the definition
                of. It will also help develop the skills of quickly reading code and determining its
                function, without the use of helpful types and variable names. This game mode has
                the added benefit of holding developers accountable for their naming conventions, as
                poorly named functions will be difficult to match with certain code blocks.
              </h4>
            </div>
            {width > 1000 &&
              <div className='col'
                style={{ paddingLeft: '50px' }}>
                <img
                  className="d-flex"
                  src="https://user-images.githubusercontent.com/46609460/231286336-006c81ba-e855-47e8-9536-6558d412b309.png"
                  alt="First slide"
                  width='450px'
                  height='450px'
                />
              </div>
            }
          </div>
        </div>
      </div>
      <div style={{ height: '100vh', display: 'block' }}>
        <br />
        <h1 className='mb-5 mt-4'>Gamemode-GitGuessr</h1>
        <br />
        <div className='container' style={{ textAlign: 'left' }}>
          <div className='row'>
            {width > 1000 &&
              <div className='col'>
                <img
                  className="d-block"
                  src='https://user-images.githubusercontent.com/46609460/231556799-5e282848-2488-4472-b44e-b10adece3e47.png'
                  alt="gg-logo"
                  height="450px"
                  width="450px"
                />
              </div>
            }
            <div className='col'>
              <br />
              <br />
              <h4>
                In the GitGuessr game mode, a player is tasked with locating the filename
                associated with a snippet of code. The user will be supplied with a snippet
                of code and a series of clickable folders and files that will allow them to
                navigate throughout the repository. Move quickly, becuase each round is only
                20 seconds long. This gamemode helps both new and experienced developers
                practice navigating a code base and will lead to better understanding and quicker
                navigation on the job.
              </h4>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
