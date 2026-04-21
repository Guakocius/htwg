import "../style/LoginForm.css";
import React, { useState } from "react";

type FormProps = {
  form: string;
}

export default function LoginForm({ form }: FormProps) {
  const [values, setValues] = useState({
    userName: "",
    password: "",
  });

  const handleInputChange = (event: React.InputEvent<HTMLInputElement>) => {
    event.preventDefault();

    const form = event.target;
    const formData = new FormData(form);

    const { name, value } = formData;
    setValues((values) => ({
      ...values,
      [name]: value,
    }));
  };

  const [submitted, setSubmitted] = useState(false);
  const [valid, setValid] = useState(false);

  const handleSubmit = (e: React.SubmitEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (values.userName && values.password) {
      setValid(true);
    }
    setSubmitted(true);
    return e;
  };

  let h3;

  if (form === "Register") {
    h3 = <h3>Registered successfully</h3>;
  } else if (form === "Login") {
    h3 = <h3>Login successful</h3>;
  } else {
    h3 = <h3>{""}</h3>;
  }

  return (
    <>
      <div className="form-container">
        <h1 className="form-header">{form}</h1>
        <form id="register-form" onSubmit={handleSubmit}>
          {submitted && valid && <div className="form-success">{h3}</div>}
          {!valid && (
            <div className="username-div">
              <label htmlFor="username">Username:</label>
              <input
                type="text"
                className="username"
                name="username"
                placeholder="Username"
                value={values.userName}
                onChange={handleInputChange}
              />
            </div>
          )}

          {submitted && !values.userName && (
            <h3 id="user-name-error">Please enter a user name</h3>
          )}

          {!valid && (
            <div className="password-div">
              <label htmlFor="password">Password:</label>
              <input
                type="password"
                className="password"
                name="password"
                placeholder="Password"
                value={values.password}
                onChange={handleInputChange}
              />
              <br />
            </div>
          )}

          {!valid && <button type="submit">{form}</button>}
        </form>
      </div>
    </>
  );
}
