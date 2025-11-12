// import SignupForm from "../components/SignupForm";

// export default function SignupPage() {
//   return (
//     <div className="">
//       <SignupForm />
//     </div>
//   );
// }

// src/pages/SignupPage.tsx
import SignupForm from "../components/SignupForm";

export default function SignupPage() {
  // top-level wrapper with a stable classname we can target in CSS
  return (
    <div className="app-root bg-white">
      <SignupForm />
    </div>
  );
}
