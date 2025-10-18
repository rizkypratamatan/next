import React, {useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {unstable_PasswordToggleField as PasswordToggleField} from "radix-ui";
import {IconEye, IconEyeClosed} from "@tabler/icons-react";
import Button from "./ui/Button.tsx";

const Login: React.FC = () => {
    const [password, setPassword] = useState<string>('');

    const onKeyDown = (event?: React.KeyboardEvent<HTMLInputElement>) => {
        if(event?.key.toLowerCase() === 'enter') {
            invoke<string>('login', {password: password}).then(result => console.log(result)).catch(console.error);
        }
    }

    const submit = (event: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
        invoke<string>('login', {password: password}).then(result => console.log(result)).catch(console.error);

        event.preventDefault();
    }

    return (
        <main className="flex h-screen">
            <section className="grow flex flex-col justify-center p-10">
                <img className="max-w-full max-h-full object-contain" src="/public/images/image-login.png"
                     alt="Login Image"/>
            </section>
            <section className="basis-145 flex flex-col gap-6 justify-center p-21.5">
                <div className="flex flex-col gap-3">
                    <h2 className="text-2xl font-bold text-text-200">Welcome to {import.meta.env.VITE_APPLICATION_NAME}!
                        👋</h2>
                    <p>Please sign-in to your account and start the adventure</p>
                </div>
                <PasswordToggleField.Root>
                    <div className="flex gap-2 items-center h-9.5 px-3.5 py-1 border border-border-100 rounded-lg">
                        <PasswordToggleField.Input
                            className="grow p-0 border-0 focus:outline-0" onChange={(event) => {
                            setPassword(event.target.value)
                        }} onKeyDown={(event) => onKeyDown(event)}/>
                        <PasswordToggleField.Toggle>
                            <PasswordToggleField.Icon
                                visible={<IconEye/>} hidden={<IconEyeClosed/>}/>
                        </PasswordToggleField.Toggle>
                    </div>
                </PasswordToggleField.Root>
                <Button className="h-9.5 bg-background-300 rounded-lg font-semibold" onClick={submit}>Login</Button>
            </section>
        </main>
    );
};

export default Login;
