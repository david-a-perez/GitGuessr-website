import { Navigate, useLocation } from "react-router-dom";

export const PrivateRoute = (props: { children: React.ReactNode }): JSX.Element => {
    const { children } = props
    const isLoggedIn: boolean = localStorage.getItem('logged_user') !== null;
    const location = useLocation()
  
    return isLoggedIn ? (
      <>{children}</>
    ) : (
      <Navigate
        replace={true}
        to="/login"
        state={{ from: `${location.pathname}${location.search}` }}
      />
    )
  }