import { useState } from "react";
import signinImage from "../assets/signinImage.png"; // same image for right side
import { useNavigate } from "react-router-dom";


export default function LoginForm() {
  const [form, setForm] = useState({ email: "", password: "" });
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState("");
    const navigate = useNavigate();


  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setLoading(true);
    setMessage("");

    try {
      const res = await fetch("http://localhost:3000/auth/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(form),
      });

      if (!res.ok) throw new Error("Login failed");

      const token = await res.text(); // ✅ token returned as plain string
      console.log("login token",token)
      localStorage.setItem("authToken", token);

      setMessage("✅ Login successful! Redirecting...");
      setTimeout(() => navigate("/user/docs"), 1200);
    } catch (err) {
      setMessage("❌ Invalid credentials or server issue.");
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="signup-wrap container p-0 bg-white">
      <div className="row bg-white">
        {/* Left side: login form */}
        <div className="col-sm-6">
          <div
            className="card border-0 p-4"
            style={{ maxWidth: 480, width: "100%", borderRadius: 14 }}
          >
            <h2 className="text-center mb-4">
              Welcome Back — Let’s Continue Collaborating!
            </h2>
            <h2 className="text-center mb-4">Login</h2>

            <form onSubmit={handleSubmit}>
              <div className="mb-3">
                <label htmlFor="email" className="form-label fw-semibold">
                  Email
                </label>
                <input
                  type="email"
                  id="email"
                  className="form-control custom-input"
                  value={form.email}
                  onChange={(e) => setForm({ ...form, email: e.target.value })}
                  placeholder="you@example.com"
                  required
                />
              </div>

              <div className="mb-4">
                <label htmlFor="password" className="form-label fw-semibold">
                  Password
                </label>
                <input
                  type="password"
                  id="password"
                  className="form-control custom-input"
                  value={form.password}
                  onChange={(e) =>
                    setForm({ ...form, password: e.target.value })
                  }
                  placeholder="••••••••"
                  required
                />
              </div>

              <button
                type="submit"
                className="btn btn-info w-100 py-2 fw-semibold"
                disabled={loading}
              >
                {loading ? "Logging in..." : "Login"}
              </button>
            </form>

            {message && (
              <div className="alert alert-info text-center mt-4 mb-0 py-2">
                {message}
              </div>
            )}
          </div>
        </div>

        {/* Right side: same image */}
        <div className="col-sm-6">
          <div
            className="signup-image"
            style={{
              backgroundImage: `url(${signinImage})`,
              backgroundSize: "cover",
              backgroundPosition: "center",
              flex: "1 1 50%",
              minHeight: "100vh",
            }}
          ></div>
        </div>
      </div>
    </div>
  );
}
