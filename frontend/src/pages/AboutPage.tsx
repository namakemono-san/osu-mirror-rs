import { Link } from "react-router-dom";

export default function AboutPage() {
    return (
        <div className="mx-auto max-w-3xl px-6 py-8 pb-12">
            <h1 className="text-3xl font-bold text-[#f5f5f5] mb-6">About</h1>

            <section className="mb-6">
                <h2 className="text-xl font-semibold text-[#e0e0e0] mb-3">Overview</h2>
                <p className="text-[#c9c9c9] leading-relaxed mb-3">
                    osu-mirror-rs is a lightweight, high-performance beatmap mirror
                    designed to provide a reliable alternative when downloading beatmaps
                    becomes slow, unstable, or inaccessible through official sources.
                </p>
                <p className="text-[#c9c9c9] leading-relaxed">
                    The project was created to offer a simple, efficient, and maintainable
                    mirror solution without the complexity or overhead often found in
                    existing systems. Built with Rust, osu-mirror-rs emphasizes speed,
                    stability, and transparency—whether self-hosted or publicly deployed.
                </p>
            </section>

            <section className="mb-6">
                <h2 className="text-xl font-semibold text-[#e0e0e0] mb-3">Key Features</h2>
                <ul className="text-[#c9c9c9] space-y-2">
                    <li className="flex items-start gap-2">
                        <span className="text-blue-400">•</span>
                        Fast beatmap delivery with efficient resource handling
                    </li>
                    <li className="flex items-start gap-2">
                        <span className="text-blue-400">•</span>
                        API compatibility with osu! API v1/v2
                    </li>
                    <li className="flex items-start gap-2">
                        <span className="text-blue-400">•</span>
                        Lightweight design that is simple to deploy and maintain
                    </li>
                    <li className="flex items-start gap-2">
                        <span className="text-blue-400">•</span>
                        Fully open-source and transparent by design
                    </li>
                    <li className="flex items-start gap-2">
                        <span className="text-blue-400">•</span>
                        Self-host friendly with no centralized infrastructure required
                    </li>
                </ul>
            </section>

            <section className="mb-6">
                <h2 className="text-xl font-semibold text-[#e0e0e0] mb-3">Philosophy</h2>
                <p className="text-[#c9c9c9] leading-relaxed mb-3">
                    osu-mirror-rs was built with a clear vision: to provide a beatmap
                    mirror that is fast, reliable, and free of unnecessary complexity.
                </p>
                <p className="text-[#c9c9c9] leading-relaxed">
                    Instead of imitating large-scale storage platforms, this project
                    focuses on clarity, predictable performance, ease of deployment, and
                    practical functionality—creating a system that developers enjoy running
                    and users enjoy relying on.
                </p>
            </section>

            <section className="mb-6">
                <h2 className="text-xl font-semibold text-[#e0e0e0] mb-3">Disclaimer</h2>
                <p className="text-[#9e9e9e] leading-relaxed">
                    This project has no affiliation with, endorsement from, or official relationship to osu! or ppy Pty Ltd.
                    Any access to osu! services is limited to publicly available web resources for the purpose of beatmap retrieval.
                    All beatmaps remain the property of their respective creators.
                </p>
            </section>

            <section>
                <h2 className="text-xl font-semibold text-[#e0e0e0] mb-3">Links</h2>
                <div className="flex flex-wrap gap-4">
                    <a
                        href="https://github.com/namakemono-san/osu-mirror-rs"
                        target="_blank"
                        rel="nofollow noreferrer"
                        className="px-4 py-2 rounded-md bg-[#393a3b] hover:bg-[#505457] text-[#f5f5f5] transition"
                    >
                        GitHub
                    </a>

                    <Link
                        to="/dmca"
                        className="px-4 py-2 rounded-md bg-[#393a3b] hover:bg-[#505457] text-[#f5f5f5] transition"
                    >
                        DMCA Policy
                    </Link>
                </div>
            </section>
        </div>
    );
}
