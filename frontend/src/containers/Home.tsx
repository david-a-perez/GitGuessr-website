import React from 'react'
import reactLogo from '../images/logo.svg'
import Carousel from 'react-bootstrap/Carousel';

export const Home = () => {

  return (
    <div style={{height: '100vh', overflowY: 'scroll', scrollSnapType: 'y mandatory'}}>
      <div style={{height: '100vh', display:'block', scrollSnapAlign: 'start'}}>
        <br />
        <h1 className='mb-5 mt-4'>Welcome to GitGuessr!</h1>
        <br />
        <div className='menu' style={{textAlign: 'left'}}>
          <div className='container'>
            <div className='row'>
              <div className='col-1' />
              <div className='col'>
                <div className="card h-80" style={{overflowY: 'hidden'}}>
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
              <div className='col' style={{paddingLeft: "10%"}}>
                <img 
                  src='https://user-images.githubusercontent.com/46609460/231286372-f3968e6c-b5c3-4e11-a1aa-22f76541830c.png' 
                  alt='logo' 
                  width='300' 
                  height='250'
                />
              </div>
              <div className='col-1' />
            </div>
          </div>
        </div>
      </div>
      <div style={{height: '100vh', display:'block', scrollSnapAlign: 'start', backgroundColor: 'lightgrey'}}>
        <br />
        <h1 className='mb-5 mt-4'>Gamemode-GitGuessr</h1>
        <div className='container' style={{textAlign: 'left'}}>
          <div className='row'>
            <div className='col'>
              <Carousel>
                <Carousel.Item>
                  <img
                    className="d-block w-100"
                    src={reactLogo}
                    alt="First slide"
                  />
                </Carousel.Item>
                <Carousel.Item>
                  <img
                    className="d-block w-100"
                    src={reactLogo}
                    alt="Second slide"
                  />
                </Carousel.Item>
                <Carousel.Item>
                  <img
                    className="d-block w-100"
                    src={reactLogo}
                    alt="Third slide"
                  />
                </Carousel.Item>
              </Carousel>
            </div>
            <div className='col'>
              <p>
                This is where I will include some text describing the gamemode.
                This will talk about gameplay and potentially the expected benefits 
                of using this gamemode.
              </p>
            </div>
          </div>
        </div>
      </div>
      <div style={{height: '100vh', display:'block', scrollSnapAlign: 'start'}}>
        <br />
        <h1 className='mb-5 mt-4'>Gamemode-Obfuscated</h1>
        <div className='container' style={{textAlign: 'left'}}>
          <div className='row'>
            <div className='col'>
              <p>
                This is where I will include some text describing the gamemode.
                This will talk about gameplay and potentially the expected benefits 
                of using this gamemode.
              </p>
            </div>
            <div className='col'>
              <Carousel>
                <Carousel.Item>
                  <img
                    className="d-flex"
                    src="https://user-images.githubusercontent.com/46609460/231286336-006c81ba-e855-47e8-9536-6558d412b309.png"
                    alt="First slide"
                    width='400' 
                    height='450'
                  />
                </Carousel.Item>
                <Carousel.Item>
                  <img
                    className="d-flex"
                    src={reactLogo}
                    alt="Second slide"
                    width='450' 
                    height='450'
                  />
                </Carousel.Item>
                <Carousel.Item>
                  <img
                    className="d-flex"
                    src={reactLogo}
                    alt="Third slide"
                    width='450' 
                    height='450'
                  />
                </Carousel.Item>
              </Carousel>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
