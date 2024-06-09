import React from "react";
import "./styles/homepage.css";
import NavBar from "../components/common/navBar.tsx";
import { faMailBulk } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
    faGithub,
    faInstagram,
} from "@fortawesome/free-brands-svg-icons";
import INFO from "../data/my_informations.ts";
import AllProjects from "../components/projects/allprojects.tsx";
import Works from "../components/homepage/works.tsx";
import Footer from "../components/common/footer.tsx";

const Homepage = () => {
    return (
        <React.Fragment>
            <div className="page-content">
                <NavBar active="home" />
                <div className="content-wrapper">
                    <div className="homepage-logo-container">
                        {/*todo add logo here*/}
                    </div>

                    <div className="homepage-container">
                        <div className="homepage-first-area">
                            <div className="homepage-first-area-left-side">
                                <div className="title homepage-title">
                                    {INFO.homepage.title}
                                </div>

                                <div className="subtitle homepage-subtitle">
                                    {INFO.homepage.description}
                                </div>
                            </div>

                            <div className="homepage-first-area-right-side">
                                <div className="homepage-image-container">
                                    <div className="homepage-image-wrapper">
                                        <img
                                            src="homepage.jpg"
                                            alt="about"
                                            className="homepage-image"
                                        />
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div className="homepage-socials">
                            <a
                                href={INFO.socials.github}
                                target="_blank"
                                rel="noreferrer"
                            >
                                <FontAwesomeIcon
                                    icon={faGithub}
                                    className="homepage-social-icon"
                                />
                            </a>
                            <a
                                href={INFO.socials.instagram}
                                target="_blank"
                                rel="noreferrer"
                            >
                                <FontAwesomeIcon
                                    icon={faInstagram}
                                    className="homepage-social-icon"
                                />
                            </a>
                            <a
                                href={`mailto:${INFO.main.email}`}
                                target="_blank"
                                rel="noreferrer"
                            >
                                <FontAwesomeIcon
                                    icon={faMailBulk}
                                    className="homepage-social-icon"
                                />
                            </a>
                        </div>

                        <div className="homepage-projects">
                            <AllProjects/>
                        </div>

                        <div className="homepage-after-title">
                            <div className="homepage-articles">
                                {/*TODO add article retrieving from backend*/}
                                {/*{myArticles.map((article, index) => (*/}
                                {/*    <div*/}
                                {/*        className="homepage-article"*/}
                                {/*        key={(index + 1).toString()}*/}
                                {/*    >*/}
                                {/*        <Article*/}
                                {/*            key={(index + 1).toString()}*/}
                                {/*            date={article().date}*/}
                                {/*            title={article().title}*/}
                                {/*            description={article().description}*/}
                                {/*            link={"/article/" + (index + 1)}*/}
                                {/*        />*/}
                                {/*    </div>*/}
                                {/*))}*/}
                            </div>

                            <div className="homepage-works">
                                <Works />
                            </div>
                        </div>

                        <div className="page-footer">
                            <Footer />
                        </div>
                    </div>
                </div>
            </div>
        </React.Fragment>
    )
}