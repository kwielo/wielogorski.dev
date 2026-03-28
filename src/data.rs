pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
    pub url: &'static str,
}

pub struct Link {
    pub label: &'static str,
    pub url: &'static str,
    pub description: &'static str,
}

pub const BIO_PARAGRAPHS: &[&str] = &[
    "Versatile, well-rounded Senior Software Engineer.",
    "I experienced Windows 95 and 98. After reinstalling Windows for the \
     1000th time, I switched to Linux. On that Windows 98, I taught myself \
     how to program. I then continued learning different languages and \
     technologies on Linux. It has been over two decades of being fascinated \
     by the amazing things you can do by writing a bunch of code.",
    "I am currently working for SwingDev by Hippo, where I can grow as \
     a Sr. Software Engineer.",
];

pub const LANGUAGES: &[&str] = &["JavaScript / TypeScript", "PHP", "Python", "Go", "SQL"];

pub const TOOLS: &[&str] = &["git", "Docker", "Postgres", "AWS"];

pub const PROJECTS: &[Project] = &[
    Project {
        name: "UUID Generator",
        description: "Quickly generate one or many UUIDs. Tracks which ones \
                      are already used and lets you copy them in one click.",
        url: "https://uuid.wielo.co/",
    },
    Project {
        name: "Pair Programming Timer",
        description: "A chess-clock for developers. Tracks who is driving \
                      and who is navigating, swapping roles on demand.",
        url: "https://double-trouble.wielo.co/",
    },
    Project {
        name: "JSON Editor",
        description: "Tweak, validate, and beautify JSON without any of \
                      your data being stored anywhere.",
        url: "https://jsonedit.wielo.co/",
    },
    Project {
        name: "Pros & Cons Notepad",
        description: "A simple, no-frills tool to list the good, the bad, \
                      and everything in between to help you make decisions.",
        url: "https://prosncons.wielo.co/",
    },
];

pub const CONTACT_LINKS: &[Link] = &[
    Link {
        label: "LinkedIn",
        url: "https://www.linkedin.com/in/kwielogorski/",
        description: "Professional profile",
    },
    Link {
        label: "GitHub",
        url: "https://github.com/kwielo",
        description: "Open-source work & projects",
    },
    Link {
        label: "Email",
        url: "mailto:contact@wielo.co",
        description: "contact@wielo.co",
    },
];

pub const WIFE_LINKS: &[Link] = &[
    Link {
        label: "wielogorska.pl",
        url: "https://wielogorska.pl/",
        description: "Marta Wielogórska — psychologist & psychotherapist",
    },
    Link {
        label: "dobryschemat.pl",
        url: "https://dobryschemat.pl/",
        description: "Dobry Schemat — psychotherapy practice",
    },
    Link {
        label: "otulanie-zyczliwoscia",
        url: "https://otulanie-zyczliwoscia.wielogorska.pl/",
        description: "Otulanie Zyczliwoscia — 8-week compassion training (CFT)",
    },
];
