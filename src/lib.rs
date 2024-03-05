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
                header .w-full .flex .flex-col ."p-8" .(bg_top) {
                    div .w-full .flex .justify-center {
                        h1 .font-bold .font-mono .text-9xl .(heading_color) ."[text-shadow:_2px_2px_3px_black]" { "felix-rath" }
                    }
                    div .w-full .flex .justify-center .gap-x-2 {
                        @let text_shadow = "[text-shadow:_2px_2px_3px_black]";
                        span .text-2xl .(text_color) .(text_shadow) {
                            // span .(heading_color) { "Software Developer." }
                            span { "Software Developer." }
                            r#"
                                I like building reliable code and systems.
                                <3 Rust, Linux, Git, NixOS, Scala, Neovim & OpenSource.
                            "#
                        }
                    }
                }
                main .(bg_main) .w-full .(text_color) .text-xl .flex .flex-col .items-center ."p-8" .pb-16 .gap-y-12 {
                    h2 .text-6xl .self-center { "Hi, I'm " span .(heading_color) { "Felix" } "! 👋" }
                    div .flex .flex-wrap .items-start .gap-x-16 .gap-y-16 .max-w-5xl {
                        img .object-scale-down .max-w-72 src="/me.jpg";
                        div ."w-1/2" .max-w-prose .grow {
                            section #about .flex .flex-col .text-justify {
                                a href="#about" {
                                    h3 .text-4xl .mb-12 .group {
                                        "— About Me"
                                        span .opacity-50 .invisible ."group-hover:visible" { " 🔗" }
                                    }
                                }
                                p .mb-6 { r#"
                                    I'm a Software Developer with a background in low-level, research-y work using Rust/C++/C, as well as high-level Web-related work in Scala, HTML, CSS, etc.
                                "# }
                                p { "I'm a big fan of strongly typed languages, and of writing edge-case free code the first time around :)" }
                            }
                            section #experience .mt-24 .flex .flex-col .text-justify {
                                a href="#experience" {
                                    h3 .text-4xl .mb-12 .group {
                                        "— Professional Experience"
                                        span .opacity-50 .invisible ."group-hover:visible" { " 🔗" }
                                    }
                                }
                                article #priceloop {
                                    @let priceloop_url = "https://priceloop.ai";
                                    h3 .text-2xl ."[text-shadow:_1px_1px_2px_black]" .(heading_color) {
                                        a href=(priceloop_url) target="_blank" {
                                            "Software Engineer | Priceloop "
                                            span .text-xl .(text_color) { "• Jan 2023 — April 2024" }
                                        }
                                    }
                                    p .mt-2 {
                                        "At "
                                        a href=(priceloop_url) target="_blank" { "Priceloop" };
                                        ", I was responsible for frontend as well as backend development."
                                        " I worked on a reactive web application in Scala (compiled to Javascript), as well as working with Scala, Postgres, AWS, and more in the backend."
                                    }
                                }
                                article #comsys .mt-12 {
                                    @let comsys_url = "https://www.comsys.rwth-aachen.de";
                                    h3 .text-2xl ."[text-shadow:_1px_1px_2px_black]" .(heading_color) {
                                        a href=(comsys_url) target="_blank" {
                                            "PhD Student | COMSYS "
                                            span .text-xl .(text_color) { "• 2016 — 2022" }
                                        }
                                    }
                                    @let klee = html! { a href="https://klee.github.io/" target="_blank" .(heading_color) { "KLEE" } };
                                    p .mt-2 {
                                        "I started - but did not finish - a PhD in Computer Science at the "
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
                                    p .mt-2 {
                                        "I did not finish my PhD for multiple reasons, the major one being mental health problems/burnout."
                                    }
                                }
                                article #education .mt-12 {
                                    h3 .text-2xl ."[text-shadow:_1px_1px_2px_black]" .(heading_color) {
                                        "Prior Education"
                                    }
                                    p .mt-2 {
                                        "I finished an M.Sc. and B.Sc. in Computer Science at "
                                        a href="https://www.rwth-aachen.de" target="_blank" .(heading_color) { "RWTH Aachen University" };
                                        " in Germany."
                                    }
                                }
                            }
                        }
                        // div .flex .flex-col .gap-y-4 .max-w-prose ."w-1/2" .grow {
                        //     @for line in LOREM_IPSUM.lines().filter(|l| l.len() > 0).take(1) {
                        //         p { (line) }
                        //     }
                        // }
                    }
                }
                footer .(bg_main) .w-full .flex .justify-center .align-center ."p-8" .text-lg .border-t-2 ."border-[#436850]" .(text_color) .gap-x-2 {
                    span { "2024 by Felix Rath" }
                    span { "•" }
                    div {
                        span { "Source for this site on " }
                        a href="https://github.com/futile/www-felix-rath.dev" target="_blank" .(heading_color) { "github ↗" };
                    }
                }
            }
        }
    }
}

#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    Response::from_html(main_page().into_string())
}

const LOREM_IPSUM: &'static str = r#"
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer vehicula ut odio eu luctus. Curabitur cursus ex vel enim scelerisque posuere. Sed purus leo, eleifend quis facilisis vitae, convallis at tortor. Suspendisse potenti. Sed condimentum, lorem eget mattis euismod, est justo malesuada leo, ornare sollicitudin felis ipsum at odio. Mauris eget lacinia ex. Morbi dignissim sapien sit amet ornare convallis. Mauris odio augue, interdum sed felis ut, egestas dictum massa. Fusce ullamcorper mattis tellus, quis cursus risus interdum quis. Suspendisse potenti. Vivamus a neque auctor, blandit enim tempor, sollicitudin quam. Nam suscipit ipsum id tortor eleifend, ut tristique lacus hendrerit. Nulla facilisi. In nec urna eget turpis pulvinar elementum. Curabitur eleifend a massa at porttitor.

Fusce vulputate, turpis vitae aliquam hendrerit, nunc risus vehicula velit, pulvinar pretium ipsum massa sed risus. Aenean sed nisi vel orci finibus tincidunt. Cras lobortis massa mi, sodales tempus tortor fermentum nec. Duis at ante ornare urna scelerisque mollis. Proin bibendum at risus eget sagittis. Curabitur suscipit turpis sed nunc facilisis consectetur. Integer ac lectus ullamcorper, semper nisi at, fringilla ligula. Fusce finibus molestie diam, sed tempus quam elementum a. Donec a mauris vel eros viverra tempor et at tellus. Pellentesque vehicula consectetur magna in tincidunt. Praesent fermentum diam ut est tincidunt, vel posuere urna commodo. Morbi ut malesuada mauris. Praesent efficitur justo mattis, mattis mauris ac, feugiat augue. Morbi cursus arcu non tempus dignissim.

Morbi nec consequat nibh, nec elementum sem. Mauris nec dolor id metus tincidunt rutrum a ut quam. Pellentesque laoreet elit et vulputate interdum. Cras purus arcu, pharetra quis sapien eu, pulvinar lacinia tortor. Cras mauris nunc, aliquam sed sollicitudin at, mattis a orci. Pellentesque gravida, sem et eleifend tristique, mauris quam maximus mauris, in porttitor tortor libero vitae mauris. Cras rutrum imperdiet urna. In hac habitasse platea dictumst. Etiam facilisis quam et nibh varius, sed sodales est rhoncus. Sed sit amet metus vitae ex tincidunt efficitur. Curabitur in sollicitudin justo, nec posuere neque. Aliquam turpis nulla, tempus quis rhoncus pellentesque, feugiat et magna. Duis pulvinar odio non urna gravida, quis dignissim nunc molestie. 
"#;
