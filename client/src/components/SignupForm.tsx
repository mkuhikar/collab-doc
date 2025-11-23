// import { useState } from "react";

// export default function SignupForm() {
//   const [form, setForm] = useState({ name: "", email: "", password: "" });
//   const [loading, setLoading] = useState(false);
//   const [message, setMessage] = useState("");

//   async function handleSubmit(e: React.FormEvent) {
//     e.preventDefault();
//     setLoading(true);
//     setMessage("");

//     try {
//       const res = await fetch("http://104.197.202.203:3000/auth/signup", {
//         method: "POST",
//         headers: { "Content-Type": "application/json" },
//         body: JSON.stringify(form),
//       });

//       if (!res.ok) throw new Error("Signup failed");

//       const data = await res.json();
//       setMessage(`✅ Welcome, ${data.name}!`);
//       setForm({ name: "", email: "", password: "" });
//     } catch (err) {
//       setMessage("❌ Something went wrong. Please try again.");
//     } finally {
//       setLoading(false);
//     }
//   }

//   return (
//     <div className="container">
//       <div className="row">
//         <div className="col-sm-6">
            
//         </div>
//         <div className="col-sm-6">

//       <div className="card  border-0 p-4" style={{ maxWidth: "420px", width: "100%" }}>
//         <h2 className="text-center mb-4 fw-bold">Create Your Account</h2>

//         <form onSubmit={handleSubmit}>
//           <div className="mb-3">
//             <label htmlFor="name" className="form-label fw-semibold">
//               Name
//             </label>
//             <input
//               type="text"
//               id="name"
//               className="form-control"
//               value={form.name}
//               onChange={(e) => setForm({ ...form, name: e.target.value })}
//               placeholder="John Doe"
//               required
//             />
//           </div>

//           <div className="mb-3">
//             <label htmlFor="email" className="form-label fw-semibold">
//               Email
//             </label>
//             <input
//               type="email"
//               id="email"
//               className="form-control"
//               value={form.email}
//               onChange={(e) => setForm({ ...form, email: e.target.value })}
//               placeholder="you@example.com"
//               required
//             />
//           </div>

//           <div className="mb-4">
//             <label htmlFor="password" className="form-label fw-semibold">
//               Password
//             </label>
//             <input
//               type="password"
//               id="password"
//               className="form-control"
//               value={form.password}
//               onChange={(e) => setForm({ ...form, password: e.target.value })}
//               placeholder="••••••••"
//               required
//             />
//           </div>

//           <button
//             type="submit"
//             className="btn btn-info w-100 py-2 fw-semibold"
//             disabled={loading}
//           >
//             {loading ? "Signing up..." : "Sign Up"}
//           </button>
//         </form>

//         {message && (
//           <div className="alert alert-info text-center mt-4 mb-0 py-2">
//             {message}
//           </div>
//         )}
//       </div>
//         </div>
//       </div>
//     </div>
//   );
// }


// src/components/SignupForm.tsx
import { useState } from "react";
import signupImage from "../assets/signupImage.png"; // ✅ import image

export default function SignupForm() {
  const [form, setForm] = useState({ name: "", email: "", password: "" });
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState("");

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setLoading(true);
    setMessage("");

    try {
  const res = await fetch("http://104.197.202.203:3000/auth/signup", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(form),
  });

  if (!res.ok) throw new Error("Signup failed");

  const data = await res.json();
  setMessage(`✅ Welcome, ${data.name}! Redirecting to login...`);

  // Clear the form
  setForm({ name: "", email: "", password: "" });

  // Redirect after short delay (so user sees message)
  setTimeout(() => {
    window.location.href = "http://104.197.202.203/auth/login";
  }, 1500);
  
} catch (err) {
  setMessage("❌ Something went wrong. Please try again.");
} finally {
  setLoading(false);
}
  }

  return (

    <div className="signup-wrap container p-0 bg-white" >
        <div className="row bg-white">
            <div className="col-sm-6"> <div
        className="card border-0 p-4"
        style={{ maxWidth: 480, width: "100%", borderRadius: 14 }}
      >
        <h2 className="text-center mb-4">Ready to start your success story in a collaborative way?</h2>
        <h2 className="text-center mb-4">Signup</h2>

        <form onSubmit={handleSubmit}>
          <div className="mb-3">
            <label htmlFor="name" className="form-label fw-semibold">
              Name
            </label>

            <input
              type="text"
              id="name"
              className="form-control custom-input"
              value={form.name}
              onChange={(e) => setForm({ ...form, name: e.target.value })}
              placeholder="John Doe"
              required
            />
          </div>

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
              onChange={(e) => setForm({ ...form, password: e.target.value })}
              placeholder="••••••••"
              required
            />
          </div>

          <button
            type="submit"
            className="btn btn-info w-100 py-2 fw-semibold"
            disabled={loading}
          >
            {loading ? "Signing up..." : "Sign Up"}
          </button>
        </form>

        {message && (
          <div className="alert alert-info text-center mt-4 mb-0 py-2">
            {message}
          </div>
        )}
      </div></div>
            <div className="col-sm-6"><div
          className="signup-image"
          style={{
            backgroundImage: `url(${signupImage})`,
            backgroundSize: "cover",
            backgroundPosition: "center",
            flex: "1 1 50%",
            minHeight: "100vh",
          }}
        ></div></div>
        </div>
     
    </div>
  );
}
