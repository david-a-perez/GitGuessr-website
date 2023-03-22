import React from 'react'

export const Home = () => {

  return (
    <div>
      <h1 className='mb-5 mt-4'>Welcome to GitGuessr!</h1>
      <div className='menu' style={{textAlign: 'left'}}>
        <div className='outer-container'>
          <div className='row'>
            <div className='col'>
              <div className="card h-80">
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
            <div className='col'>
              <img src='https://user-images.githubusercontent.com/46609460/220524085-2e913612-03a5-431e-a326-013cd66d10bf.png' alt='logo' width='450' height='300'/>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
