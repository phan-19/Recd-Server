import React, { useState } from 'react';
import './UserSignup.css';

type User = {
  user_id: number;
  username: string;
  bio: string;
};

type UserSignupProps = {
  onSignup: (user: User) => void;
};

const UserSignup: React.FC<UserSignupProps> = ({onSignup}) => {
    const [username, setUsername] = useState(''); //p
    const [password, setPassword] = useState(''); //vrysecurepassword:D
    const [bio, setBio] = useState('');
    const [message, setMessage] = useState('');

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        var url = `http://66.231.155.18:3000/user`;

        try {
            const options = {
                method: 'POST',
     	        headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ username, password, bio })
            };

            var response = await fetch(url, options);

            if (!response.ok) {
                throw new Error(`HTTP error! Status: ${response.status}`);
            }

            const result = await response.json();

            if (result.status === 'success') {
                const newUser: User = {
                    user_id: result.user_id,
                    username,
                    bio
                }
                setMessage('Signup was successful');
                onSignup(newUser);
            } else {
                setMessage('Invalid username or password.');
            }
        } catch (error) {
            console.error("Signup error:", error);
            setMessage("Could not connect to server. Please try again.");
        }
    };

    return (
        <form className="user-signup" onSubmit={handleSubmit}>
            <h2 className="signup-title">Sign Up</h2>
            <input
                type="text"
                placeholder="username"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                required
            />
            <input
                type="text"
                placeholder="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required
            />
            <input
                type="text"
                placeholder="bio"
                value={bio}
                onChange={(e) => setBio(e.target.value)}
                required
             />
            <button type="submit">Sign Up</button>
            {message && <p>{message}</p>}
        </form>
    );
};

export default UserSignup;
