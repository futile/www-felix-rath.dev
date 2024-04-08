use maud::{html, Markup, DOCTYPE};
use worker::*;

fn main_page() -> Markup {
    let bg_top = "bg-[#436850]";
    let bg_main = "bg-[#12372A]";
    let text_color = "text-[#ADBC9F]";
    let heading_color = "text-[#FBFADA]";
    let text_dark = "text-[#12372A]";

    html! {
        (DOCTYPE)
        // reference for the height-stuff: https://dev.to/fenok/stretching-body-to-full-viewport-height-the-missing-way-2ghd
        html .min-h-full .flex .flex-col {
            head {
                meta charset="utf-8";

                // Moz on viewport: https://developer.mozilla.org/en-US/docs/Web/HTML/Viewport_meta_tag
                meta name="viewport" content="width=device-width, initial-scale=1";

                title { "felix-rath.dev | Software Developer" }

                link rel="preconnect" href="https://rsms.me/";
                link rel="stylesheet" type="text/css" href="https://rsms.me/inter/inter.css";

                link rel="stylesheet" type="text/css" href="/out.css";

                link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png";
                link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png";
                link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png";
                link rel="manifest" href="/site.webmanifest";
            }
            body .grow .grid .grid-columns-1 ."[grid-template-rows:_auto_1fr_auto_auto]" .w-full .h-full .justify-stretch {
                header .w-full .flex .flex-col ."p-2" ."pb-4" ."sm:p-8" ."sm:pb-6" .(bg_top) .justify-center .items-center ."sm:gap-y-2" {
                    h1 ."font-[JetBrainsMono]" .font-bold ."text-[57px]" ."sm:text-9xl" .(heading_color) ."[text-shadow:_2px_2px_3px_black]" { "felix-rath" }
                    span ."text-[22px]" ."sm:text-4xl" .(text_color) ."[text-shadow:_2px_2px_3px_black]" { "I build reliable code and systems" }
                }
                main .(bg_main) .w-full .(text_color) .text-base ."sm:text-xl" .flex .flex-col .items-center ."p-2" ."pt-4" ."pb-8" ."gap-y-6" ."sm:p-8" ."sm:pt-6" ."sm:pb-16" ."sm:gap-y-12" {
                    h2 ."text-4xl" ."sm:text-6xl" .self-center { "Hi, I'm " span .(heading_color) { "Felix" } "! 👋" }
                    div .flex .flex-wrap .items-start .gap-16 .max-w-5xl .self-center {
                        // TODO: proper resizing (when the site is resized)
                        div .flex .flex-col .gap-y-12 .items-center .justify-center .justify-self-center .grow ."sm:grow-0" {
                            img .object-scale-down .max-w-72 .self-center src="/me.jpg";
                            div .flex .gap-4 .justify-evenly .max-w-72 .w-full {
                                (contact_icons())
                            }
                        }
                        div ."w-1/2" .max-w-prose .grow {
                            @let heading_size = "text-2xl";
                            @let sm_heading_size = "sm:text-4xl";
                            @let sub_heading_size = "text-xl";
                            @let sm_sub_heading_size = "sm:text-3xl";
                            @let sub2_heading_size = "text-lg";
                            @let sm_sub2_heading_size = "sm:text-2xl";
                            section #about .flex .flex-col .text-justify .gap-6 {
                                a href="#about" {
                                    h3 .(heading_size) .(sm_heading_size) .mb-6 .group {
                                        "— About Me"
                                        span .opacity-50 .invisible ."group-hover:visible" { " 🔗" }
                                    }
                                }
                                p { r#"
                                    I am a Software Developer with a strong background in low-level, research-y work using Rust/C++/C, as well as in high-level Web-related work in Scala(JS), Rust, HTML, CSS, etc.
                                "# }
                                p { r#"
                                    New programming languages and technologies are things I pick up quickly, and I enjoy working with modern stacks and efficient technologies most.
                                    Recently I have also started to enjoy frontend work a lot, especially Web-related.
                                "# }
                                p { r#"
                                    In the past I have contributed to various open-source projects, including, e.g., the Rust-compiler and NixOS/nixpkgs.
                                    I majorly enjoy engaging with, and participating in, all the communities that open-source has produced!
                                "# }
                                p { r#"
                                    I am a big fan of strongly typed languages, correctness, and of covering as many edge cases as possible before (my) code hits production.
                                    I also like to work in a team and understand the value of clear, early and regular communication and feedback, from other developers as well as from more design- and/or product-related roles.
                                "# }
                                p { r#"
                                    In my free time I like to travel (sometimes Digital Nomad-style), go bouldering or climbing, and read up on the newest programming languages and technologies.
                                    My main OS nowadays is NixOS, which I really enjoy.
                                    You can check out my system configuration 
                                    "#
                                    a href="https://github.com/futile/nixos-config" .(heading_color) { "here" }
                                    "."
                                }
                            }
                            section #experience .mt-16 ."sm:mt-32" .flex .flex-col .text-justify {
                                a href="#experience" {
                                    h3 .(heading_size) .(sm_heading_size) .mb-12 .group {
                                        "— Professional Experience"
                                        span .opacity-50 .invisible ."group-hover:visible" { " 🔗" }
                                    }
                                }
                                article #priceloop {
                                    @let priceloop_url = "https://priceloop.ai";
                                    h3 .(sub_heading_size) .(sm_sub_heading_size) .(heading_color) {
                                        a href=(priceloop_url) target="_blank" {
                                            "Software Engineer | Priceloop "
                                            span .(sub2_heading_size) .(sm_sub2_heading_size) .(text_color) { "• Jan 2023 — April 2024" }
                                        }
                                    }
                                    p .mt-4 {
                                        // TODO: Improve description a bit :/
                                        "At "
                                        a href=(priceloop_url) target="_blank" { "Priceloop" };
                                        ", I was responsible for frontend as well as backend development."
                                        " I worked on a reactive web application in Scala (compiled to Javascript), as well as working with Scala, Postgres, AWS, and more in the backend."
                                    }
                                }
                                article #comsys .mt-12 {
                                    @let comsys_url = "https://www.comsys.rwth-aachen.de";
                                    h3 .(sub_heading_size) .(sm_sub_heading_size) .(heading_color) {
                                        a href=(comsys_url) target="_blank" {
                                            "PhD Student | COMSYS "
                                            span .(sub2_heading_size) .(sm_sub2_heading_size) .(text_color) { "• 2016 — 2022" }
                                        }
                                    }
                                    @let klee = html! { a href="https://klee.github.io/" target="_blank" .(heading_color) { "KLEE" } };
                                    p .mt-4 {
                                        "I started (but did not finish) - a PhD in Computer Science at the "
                                        a href=(comsys_url) target="_blank" .(heading_color) { "Chair of Communication and Distributed Systems at RWTH Aachen University." };
                                        " My research was focused on automated testing and reliability for (distributed) software, particularly using Symbolic Execution with " (klee) "."
                                    }
                                    p .mt-2 {
                                        "During my time as a PhD student, I"
                                        ul .list-disc .list-inside .ml-8 .mt-2 .space-y-2 {
                                            li {
                                                "Worked on "
                                                a href="https://dl.acm.org/doi/pdf/10.1145/3123878.3131977" target="_blank" .(heading_color) { "predicting the performance of network functions written in eBPF" }
                                                "."
                                            }
                                            li {
                                                "Published a paper on "
                                                a href="https://arxiv.org/pdf/1811.12099.pdf" target="_blank" .(heading_color) { "Interoperability-Guided Testing of QUIC Implementations" }
                                                "."
                                            }
                                            li {
                                                "Contributed "
                                                a href="https://github.com/klee/klee/pull/966#issuecomment-632694431" target="_blank" .(heading_color) { "Open-Source Support for Analyzing C++-Programs" }
                                                " to the "
                                                (klee) " Symbolic Execution Engine"
                                                ". I also gave a talk on this at the "
                                                a href="https://srg.doc.ic.ac.uk/klee21/" target="_blank" .(heading_color) { "2021 KLEE Workshop" }
                                                ": "
                                                a href="https://srg.doc.ic.ac.uk/klee21/talks/Rath-Cpp.pdf" target="_blank" .(heading_color) { "slides" }
                                                ", "
                                                a href="https://youtu.be/xaWE9rJzHVo" target="_blank" .(heading_color) { "video" }
                                                "."
                                            }
                                            li {
                                                "Presented my research on "
                                                a href="https://srg.doc.ic.ac.uk/klee18/talks/Rath-Pointer-Tracking.pdf" target="_blank" .(heading_color) { "A Pointer-Tracking Memory Model For KLEE & Symbolic Execution" }
                                                " at the "
                                                a href="https://srg.doc.ic.ac.uk/klee18/" target="_blank" .(heading_color) { "2018 KLEE Workshop" }
                                                "."
                                            }
                                        }
                                    }
                                }
                                article #education .mt-12 {
                                    h3 .(sub_heading_size) .(sm_sub_heading_size) .(heading_color) {
                                        "Prior Education"
                                    }
                                    p .mt-4 {
                                        // TODO: add info (and maybe PDFs?) about my theses
                                        "I finished an M.Sc. and B.Sc. in Computer Science at "
                                        a href="https://www.rwth-aachen.de" target="_blank" .(heading_color) { "RWTH Aachen University" };
                                        " in Germany."
                                    }
                                }
                            }
                        }
                    }
                }
                footer .(bg_main) .w-full .flex .flex-wrap .justify-center .align-center ."p-2" ."py-6" ."sm:p-8" .text-lg .border-t-2 ."border-[#436850]" .(text_color) .gap-2 {
                    div {
                        span { "Source for this site on " }
                        a href="https://github.com/futile/www-felix-rath.dev" target="_blank" .(heading_color) { "github ↗" };
                    }
                    span { "•" }
                    div .flex .flex-wrap .gap-x-2 {
                        (contact_icons())
                    }
                    span { "•" }
                    span .text-center { "<3 Rust, Linux, Git, NixOS, Scala, Neovim & OpenSource" }
                }
            }
        }
    }
}

fn contact_icons() -> Markup {
    let icon_color = "#ADBC9F";
    let icon_width = "w-6";

    html! {
        a href="https://github.com/futile" target="_blank" title="GitHub Profile" {
            // icon: fontawesome.com -> square-github
            svg .(icon_width) xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" alt="GitHub logo" {
                path fill=(icon_color) d="M448 96c0-35.3-28.7-64-64-64H64C28.7 32 0 60.7 0 96V416c0 35.3 28.7 64 64 64H384c35.3 0 64-28.7 64-64V96zM265.8 407.7c0-1.8 0-6 .1-11.6c.1-11.4 .1-28.8 .1-43.7c0-15.6-5.2-25.5-11.3-30.7c37-4.1 76-9.2 76-73.1c0-18.2-6.5-27.3-17.1-39c1.7-4.3 7.4-22-1.7-45c-13.9-4.3-45.7 17.9-45.7 17.9c-13.2-3.7-27.5-5.6-41.6-5.6s-28.4 1.9-41.6 5.6c0 0-31.8-22.2-45.7-17.9c-9.1 22.9-3.5 40.6-1.7 45c-10.6 11.7-15.6 20.8-15.6 39c0 63.6 37.3 69 74.3 73.1c-4.8 4.3-9.1 11.7-10.6 22.3c-9.5 4.3-33.8 11.7-48.3-13.9c-9.1-15.8-25.5-17.1-25.5-17.1c-16.2-.2-1.1 10.2-1.1 10.2c10.8 5 18.4 24.2 18.4 24.2c9.7 29.7 56.1 19.7 56.1 19.7c0 9 .1 21.7 .1 30.6c0 4.8 .1 8.6 .1 10c0 4.3-3 9.5-11.5 8C106 393.6 59.8 330.8 59.8 257.4c0-91.8 70.2-161.5 162-161.5s166.2 69.7 166.2 161.5c.1 73.4-44.7 136.3-110.7 158.3c-8.4 1.5-11.5-3.7-11.5-8zm-90.5-54.8c-.2-1.5 1.1-2.8 3-3.2c1.9-.2 3.7 .6 3.9 1.9c.3 1.3-1 2.6-3 3c-1.9 .4-3.7-.4-3.9-1.7zm-9.1 3.2c-2.2 .2-3.7-.9-3.7-2.4c0-1.3 1.5-2.4 3.5-2.4c1.9-.2 3.7 .9 3.7 2.4c0 1.3-1.5 2.4-3.5 2.4zm-14.3-2.2c-1.9-.4-3.2-1.9-2.8-3.2s2.4-1.9 4.1-1.5c2 .6 3.3 2.1 2.8 3.4c-.4 1.3-2.4 1.9-4.1 1.3zm-12.5-7.3c-1.5-1.3-1.9-3.2-.9-4.1c.9-1.1 2.8-.9 4.3 .6c1.3 1.3 1.8 3.3 .9 4.1c-.9 1.1-2.8 .9-4.3-.6zm-8.5-10c-1.1-1.5-1.1-3.2 0-3.9c1.1-.9 2.8-.2 3.7 1.3c1.1 1.5 1.1 3.3 0 4.1c-.9 .6-2.6 0-3.7-1.5zm-6.3-8.8c-1.1-1.3-1.3-2.8-.4-3.5c.9-.9 2.4-.4 3.5 .6c1.1 1.3 1.3 2.8 .4 3.5c-.9 .9-2.4 .4-3.5-.6zm-6-6.4c-1.3-.6-1.9-1.7-1.5-2.6c.4-.6 1.5-.9 2.8-.4c1.3 .7 1.9 1.8 1.5 2.6c-.4 .9-1.7 1.1-2.8 .4z";
            }
        }
        a href="https://www.linkedin.com/in/felix-rath/" target="_blank" title="LinkedIn Profile" {
            // icon: fontawesome.com -> linkedin
            svg .(icon_width) xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" {
                path fill=(icon_color) d="M416 32H31.9C14.3 32 0 46.5 0 64.3v383.4C0 465.5 14.3 480 31.9 480H416c17.6 0 32-14.5 32-32.3V64.3c0-17.8-14.4-32.3-32-32.3zM135.4 416H69V202.2h66.5V416zm-33.2-243c-21.3 0-38.5-17.3-38.5-38.5S80.9 96 102.2 96c21.2 0 38.5 17.3 38.5 38.5 0 21.3-17.2 38.5-38.5 38.5zm282.1 243h-66.4V312c0-24.8-.5-56.7-34.5-56.7-34.6 0-39.9 27-39.9 54.9V416h-66.4V202.2h63.7v29.2h.9c8.9-16.8 30.6-34.5 62.9-34.5 67.2 0 79.7 44.3 79.7 101.9V416z";
            }
        }
        a href="https://scholar.google.com/citations?user=eSAXVJwAAAAJ&hl=en" target="_blank" title="Google Scholar Profile" {
            // icon: fontawesome.com -> google-scholar
            svg .(icon_width) xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" {
                path fill=(icon_color) d="M390.9 298.5c0 0 0 .1 .1 .1c9.2 19.4 14.4 41.1 14.4 64C405.3 445.1 338.5 512 256 512s-149.3-66.9-149.3-149.3c0-22.9 5.2-44.6 14.4-64h0c1.7-3.6 3.6-7.2 5.6-10.7c4.4-7.6 9.4-14.7 15-21.3c27.4-32.6 68.5-53.3 114.4-53.3c33.6 0 64.6 11.1 89.6 29.9c9.1 6.9 17.4 14.7 24.8 23.5c5.6 6.6 10.6 13.8 15 21.3c2 3.4 3.8 7 5.5 10.5zm26.4-18.8c-30.1-58.4-91-98.4-161.3-98.4s-131.2 40-161.3 98.4L0 202.7 256 0 512 202.7l-94.7 77.1z";
            }
        }
        a href="https://mas.to/@frath" target="_blank" title="Mastodon Profile" {
            // icon: fontawesome.com -> mastodon
            svg .(icon_width) xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" {
                path fill=(icon_color) d="M433 179.1c0-97.2-63.7-125.7-63.7-125.7-62.5-28.7-228.6-28.4-290.5 0 0 0-63.7 28.5-63.7 125.7 0 115.7-6.6 259.4 105.6 289.1 40.5 10.7 75.3 13 103.3 11.4 50.8-2.8 79.3-18.1 79.3-18.1l-1.7-36.9s-36.3 11.4-77.1 10.1c-40.4-1.4-83-4.4-89.6-54a102.5 102.5 0 0 1 -.9-13.9c85.6 20.9 158.7 9.1 178.8 6.7 56.1-6.7 105-41.3 111.2-72.9 9.8-49.8 9-121.5 9-121.5zm-75.1 125.2h-46.6v-114.2c0-49.7-64-51.6-64 6.9v62.5h-46.3V197c0-58.5-64-56.6-64-6.9v114.2H90.2c0-122.1-5.2-147.9 18.4-175 25.9-28.9 79.8-30.8 103.8 6.1l11.6 19.5 11.6-19.5c24.1-37.1 78.1-34.8 103.8-6.1 23.7 27.3 18.4 53 18.4 175z";
            }
        }
        a href="mailto:felixm.rath@gmail.com" target="_blank" title="Email Address" {
            // icon: fontawesome.com -> square-envelope
            svg .(icon_width) xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" {
                path fill=(icon_color) d="M64 32C28.7 32 0 60.7 0 96V416c0 35.3 28.7 64 64 64H384c35.3 0 64-28.7 64-64V96c0-35.3-28.7-64-64-64H64zM218 271.7L64.2 172.4C66 156.4 79.5 144 96 144H352c16.5 0 30 12.4 31.8 28.4L230 271.7c-1.8 1.2-3.9 1.8-6 1.8s-4.2-.6-6-1.8zm29.4 26.9L384 210.4V336c0 17.7-14.3 32-32 32H96c-17.7 0-32-14.3-32-32V210.4l136.6 88.2c7 4.5 15.1 6.9 23.4 6.9s16.4-2.4 23.4-6.9z";
            }
        }
    }
}

#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    Response::from_html(main_page().into_string())
}
