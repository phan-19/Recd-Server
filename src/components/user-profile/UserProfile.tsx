import React, { useState, useEffect } from 'react';
import './UserProfile.css'

import UserLogin from '../user-login/UserLogin';

type User = {
    user_id: number,
    username: string
};

type Profile = {
    user_id: number,
    username: string,
    bio: string
}

const UserProfile: React.FC = () => {
    const [user, setUser] = useState<User | null>(null);
    const [profile, setProfile] = useState<Profile | null>(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const stored = localStorage.getItem('user');
        if (stored) {
            const storedUser: User = JSON.parse(stored);
            setUser(storedUser);

            fetch(`http://66.231.155.18:3000/page/user/${storedUser.user_id}`)
                .then(response => response.json())
                .then(data => {
                    setProfile(data);
                })
                .catch(err => {
                    console.error('Failed to fetch profile:', err);
                });
            setLoading(false);
        } else {
            setLoading(false);
        }
    }, []);

    const handleLogin = (user: User) => {
        setUser(user);
        localStorage.setItem('user', JSON.stringify(user));
    };

    const handleLogout = () => {
        localStorage.removeItem('user');
        setUser(null);
        setProfile(null);
        window.location.reload();
    };

    if (loading) {
        return <p>Loading...</p>; 
    }

    if (!user) {
        return <UserLogin onLogin={handleLogin} />;
    }

    if (!profile) {
        return <p>Loading profile...</p>;
    }

    return (
        <div className="profile">
            <div className='profile-content'>
                <h2 className='username'>{'@'}{profile.username}</h2>
                <p className='bio'>{profile.bio}</p>
                <button onClick={handleLogout}>Log Out</button>
            </div>
        </div>
    );
}

export default UserProfile;
