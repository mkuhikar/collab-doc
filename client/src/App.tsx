
// src/App.tsx
import { BrowserRouter as Router, Routes, Route, Navigate } from "react-router-dom";
import LoginPage from "../src/pages/LoginPages";
import SignupPage from "../src/pages/SignupPages";
import UserDocs from "../src/pages/UserDocs";
import EditorPage from "./pages/EditorPage";

export default function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<LoginPage />} />
        <Route path="/auth/login" element={<LoginPage />} />
        <Route path="/auth/signup" element={<SignupPage />} />
        <Route path="/user/docs" element={<UserDocs />} />
        <Route path="/editor/new" element={<EditorPage />} />
        <Route path="/editor/:id" element={<EditorPage />} />


        {/*Any other path redirects to login */}
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </Router>
  );
}

