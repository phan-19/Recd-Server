import React, { useState, useEffect } from 'react';
import './UserProfile.css'
import Card from "../../components/assets/card/Card";

import UserLogin from '../user-login/UserLogin';

type User = {
    user_id: number,
    username: string
};

type Profile = {
    user_id: number,
    username: string,
    bio: string,
    reviews: number[]
}

type UserProps = {
    user_id: number
}

const UserProfile: React.FC<UserProps> = ({ user_id }) => {
    const [user, setUser] = useState<User | null>(null);
    const [profile, setProfile] = useState<Profile | null>(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const stored = localStorage.getItem('user');
        if (stored) {
            const storedUser: User = JSON.parse(stored);
            setUser(storedUser);

            fetch(`http://66.231.155.18:3000/page/user/${user_id}`)
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

    const logOutButton = () => {
        if (user.user_id === profile.user_id) {
            return (<button onClick={handleLogout}>Log Out</button>)
        }
        return (<></>)
    };

    return (
        <div className="profile">
            <div className='profile-content'>
                <h2 className='username'>{'@'}{profile.username}</h2>
                <p className='bio'>{profile.bio}</p>
                <Card.CardScroll reviews={profile.reviews} />
                <>{logOutButton}</>
            </div>
        </div>
    );
}

export default UserProfile;
