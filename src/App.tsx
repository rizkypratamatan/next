import React from "react";
import {createBrowserRouter, createRoutesFromElements, Route, RouterProvider} from "react-router";
import Login from "./components/Login.tsx";

const App: React.FC = () => {
    const router = createBrowserRouter(
        createRoutesFromElements(
            <Route>
                <Route path="/" element={<Login/>}/>
            </Route>
        )
    );

    return (
        <RouterProvider router={router}></RouterProvider>
    );
}

export default App;
