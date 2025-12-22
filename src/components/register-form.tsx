import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Field,
  FieldDescription,
  FieldGroup,
  FieldLabel,
} from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { useForm } from "react-hook-form";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router";
import { store } from "@/utils/token";

type RegisterFormValues = {
  username: string;
  email: string;
  password: string;
};

type AuthResponse = {
  token: string;
  user: {
    id: string;
    username: string;
    email: string;
    password: string;
    role: string;
    updated_at: string;
    created_at: string;
  };
};

export function RegisterForm({
  className,
  ...props
}: React.ComponentProps<"div">) {
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<RegisterFormValues>();
  let navigate = useNavigate();

  const onSubmit = async (data: RegisterFormValues) => {
    try {
      const res: AuthResponse = await invoke("register", {
        input: {
          username: data.username,
          email: data.email,
          password: data.password,
        },
      });

      await store.set("token", res.token);
      await store.save();

      navigate("/");

      console.log(res);
    } catch (err) {
      console.log(err);
    }
  };

  return (
    <div className={cn("w-105 flex flex-col gap-6", className)} {...props}>
      <Card>
        <CardHeader>
          <CardTitle>Create your account</CardTitle>
          <CardDescription>
            Enter your details below to create a new account
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit(onSubmit)}>
            <FieldGroup>
              <Field>
                <FieldLabel htmlFor="username">Username</FieldLabel>
                <Input
                  id="username"
                  placeholder="your username"
                  {...register("username", {
                    required: "Username is required",
                  })}
                />
                {errors.username && (
                  <FieldDescription className="text-red-500">
                    {errors.username.message}
                  </FieldDescription>
                )}
              </Field>

              <Field>
                <FieldLabel htmlFor="email">Email</FieldLabel>
                <Input
                  id="email"
                  type="email"
                  placeholder="m@example.com"
                  {...register("email", {
                    required: "Email is required",
                    pattern: {
                      value: /\S+@\S+\.\S+/,
                      message: "Invalid email address",
                    },
                  })}
                />
                {errors.email && (
                  <FieldDescription className="text-red-500">
                    {errors.email.message}
                  </FieldDescription>
                )}
              </Field>

              <Field>
                <FieldLabel htmlFor="password">Password</FieldLabel>
                <Input
                  id="password"
                  type="password"
                  {...register("password", {
                    required: "Password is required",
                  })}
                />
                {errors.password && (
                  <FieldDescription className="text-red-500">
                    {errors.password.message}
                  </FieldDescription>
                )}
              </Field>

              <Field>
                <Button type="submit" disabled={isSubmitting}>
                  {isSubmitting ? "Registering..." : "Register"}
                </Button>
                <FieldDescription className="text-center">
                  Already have an account? <a href="/login">Login</a>
                </FieldDescription>
              </Field>
            </FieldGroup>
          </form>
        </CardContent>
      </Card>
    </div>
  );
}
