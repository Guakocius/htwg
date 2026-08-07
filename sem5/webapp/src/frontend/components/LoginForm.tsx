import "../../style/LoginForm.css";
import React, { useState } from "react";
import axios from "axios";

import type { FormProps } from "../../types.ts";

/**
 * @param form - The kind of form (Register, Login) being used
 */
export default function LoginForm({ form }: FormProps) {
  const [values, setValues] = useState({
    username: "",
    password: "",
    confirmPassword: "",
    terms: false,
  });

  const [valid, setValid] = useState<Record<string, string>>({});

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();

    const { name, value, type, checked } = e.target as HTMLInputElement;
    setValues((values) => ({
      ...values,
      [name]: type === "checkbox" ? checked : value,
    }));
    if (valid[name]) {
      setValid((values) => ({
        ...values,
        [name]: "",
      }));
    }
  };

  const validate = () => {
    const newErrors: Record<string, string> = {};

    if (!values.username.trim()) {
      newErrors.username = "Username is required";
    }

    if (!values.password) {
      newErrors.password = "Password is required";
    } else if (values.password.length < 8) {
      newErrors.password = "Password must be at least 8 characters";
    }

    if (form === "Register") {
      if (values.password && values.password.length < 8) {
        newErrors.password = "Password must be at least 8 characters";
      }
      if (values.password !== values.confirmPassword) {
        newErrors.confirmPassword = "Passwords do not match";
      }
      if (!values.terms) {
        newErrors.terms = "You must accept the terms and conditions";
      }
    }

    setValid(newErrors);
    return newErrors;
  };

  const handleSubmit = async (e: React.SubmitEvent<HTMLFormElement>) => {
    e.preventDefault();
    const validationErrors = validate();
    const hasErrors = Object.keys(validationErrors).length > 0;

    if (hasErrors) {
      console.log(
        "Validation failed. Please fix the errors before submitting:",
        validationErrors,
      );
      return;
    }

    try {
      const payload = {
        username: values.username,
        password: values.password,
      };

      if (form === "Register") {
        const resp = await axios.post(
          "http://localhost:5000/api/users/register",
          payload,
        );
        if (resp.status === 201) {
          alert("Registration successful!");
        }
      } else {
        const resp = await axios.post(
          "http://localhost:5000/api/users/login",
          payload,
        );
        if (resp.status === 200) {
          sessionStorage.setItem("token", resp.data.token);
          alert(`Welcome back, ${resp.data.user.username}.`);
        }
      }
    } catch (e: any) {
      console.error("Error submitting form:", e);
      alert(
        e.response?.data?.message ||
          "Something went wrong during registration.",
      );
    }
  };

  return (
    <>
      <div className="form-container">
        <h1 className="form-header">{form}</h1>
        <form id="register-form" onSubmit={handleSubmit}>
          <div className="username-div">
            <label htmlFor="username">Username:</label>
            <input
              type="text"
              className="username"
              name="username"
              placeholder="Username"
              value={values.username}
              onChange={handleInputChange}
            />
            {valid.userName && (
              <span style={{ color: "red" }}>{valid.username}</span>
            )}
          </div>

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
            {valid.password && (
              <span style={{ color: "red" }}>{valid.password}</span>
            )}
          </div>
          {form === "Register" && (
            <div className="password-div">
              <label htmlFor="confirmPassword">Confirm Password</label>
              <input
                type="password"
                className="password"
                name="confirmPassword"
                placeholder="Confirm Password"
                value={values.confirmPassword}
                onChange={handleInputChange}
              />
              {valid.confirmPassword && (
                <span style={{ color: "red" }}>{valid.confirmPassword}</span>
              )}
            </div>
          )}
          {form === "Register" && (
            <div className="terms-div">
              <label htmlFor="terms">
                <input
                  type="checkbox"
                  className="terms"
                  name="terms"
                  checked={values.terms}
                  onChange={handleInputChange}
                />
                I accept the terms and conditions
              </label>
              {valid.terms && (
                <span style={{ color: "red" }}>{valid.terms}</span>
              )}
            </div>
          )}
          <button type="submit">{form}</button>
        </form>
      </div>
    </>
  );
}
