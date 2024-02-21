'use client';

// Map of links to display in the side navigation.
// Depending on the size of the application, this would be stored in a database.
import React from "react";

export const links = [
    { name: 'Home', href: '/dashboard'},
    {
        name: 'Services',
        href: '/dashboard/services'
    },
    { name: 'Clients', href: '/dashboard/clients' },
];

export const usePathname = () => {
    // todo add path shenanigans here
    // todo this should then be used to set classname of each link depending its active or not
    return "path";
}

export default function NavLinks() {
    const pathname = usePathname();
    return (
            links.map((link) => {
                return (
                    <a
                        key={link.name}
                        href={link.href}
                        className='flex h-[48px] grow items-center justify-center gap-2 rounded-md bg-gray-50 p-3 text-sm font-medium hover:bg-sky-100 hover:text-blue-600 md:flex-none md:justify-start md:p-2 md:px-3'
                    >
                        <p className="hidden md:block">{link.name}</p>
                    </a>
                );
            })
    );
}
