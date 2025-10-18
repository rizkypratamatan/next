import React, {MouseEventHandler} from "react";


type Props = {
    children: React.ReactNode;
    className?: string;
    type?: 'submit' | 'reset' | 'button' | undefined;
    onClick?: MouseEventHandler<HTMLButtonElement>;
};

const Button: React.FC<Props> = ({children, className, type, onClick}) => {
    return <button className={className} type={type} onClick={onClick}>{children}</button>;
};

export default Button;
