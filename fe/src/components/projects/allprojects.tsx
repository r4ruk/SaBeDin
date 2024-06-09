import React from "react";

import Project from "./project";

import "./styles/allprojects.css";

const AllProjects = () => {
    return (
        <div className="all-projects-container">
            {/*TODO load projects from backend via lib*/}
            {/*{INFO.projects.map((project, index) => (*/}
            {/*    <div className="all-projects-project" key={index}>*/}
            {/*        <Project*/}
            {/*            logo={project.logo}*/}
            {/*            title={project.title}*/}
            {/*            description={project.description}*/}
            {/*            linkText={project.linkText}*/}
            {/*            link={project.link}*/}
            {/*        />*/}
            {/*    </div>*/}
            {/*))}*/}
        </div>
    );
};

export default AllProjects;