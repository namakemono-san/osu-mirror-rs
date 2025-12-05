import { createRoot } from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";

import "./index.css";

import MainLayout from "./layouts/MainLayout";
import BeatmapsPage from "./pages/BeatmapsPage";
import AboutPage from "./pages/AboutPage";
import DMCAPage from "./pages/DMCAPage";
import NotFoundPage from "./pages/NotFoundPage";
import ErrorPage from "./pages/ErrorPage";

const router = createBrowserRouter([
  {
    path: "/",
    element: <MainLayout />,
    errorElement: <ErrorPage />,
    children: [
      {
        index: true,
        element: <BeatmapsPage />,
      },
      {
        path: "beatmapsets/:id",
        element: <BeatmapsPage />,
      },
      {
        path: "about",
        element: <AboutPage />,
      },
      {
        path: "dmca",
        element: <DMCAPage />,
      },
      {
        path: "*",
        element: <NotFoundPage />,
      },
    ],
  },
]);

createRoot(document.getElementById("root")!).render(
  <RouterProvider router={router} />
);