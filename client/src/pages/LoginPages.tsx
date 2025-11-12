import LoginForm from "../components/LoginForm";

export default function LoginPage() {
  // top-level wrapper with a stable classname we can target in CSS
  return (
    <div className="app-root bg-white">
      <LoginForm />
    </div>
  );
}
