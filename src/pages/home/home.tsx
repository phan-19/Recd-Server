import './home.css';

import Card from "../../components/assets/card/Card";
import { useState, useEffect } from 'react';

export default function Home() {
    const [reviews, setReviews] = useState([]);

    useEffect(() => {
        const fetchReviews = async () => {
            var url = `http://66.231.155.18:3000/page/home`;
            console.log(url);
            var response = await fetch(url);
            var result = await response.json();
            setReviews(result.reviews);
        }
        fetchReviews();
    }, []);

    return (
        <main>
            <div>
                <h2 className='section-title'>
                    Recommended
                </h2>
                <Card.CardScroll reviews={reviews} />
                <h2 className='section-title'>
                    Recently Added
                </h2>
                <div className='card-scroll'>

                </div>
            </div>
        </main>

    );
}