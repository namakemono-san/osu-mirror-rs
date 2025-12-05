export default function DMCAPage() {
    return (
        <div className="mx-auto max-w-3xl px-6 py-12 pb-24">
            <h1 className="text-3xl font-bold text-[#f5f5f5] mb-8">DMCA Policy</h1>

            <section className="mb-8">
                <h2 className="text-xl font-semibold text-[#e0e0e0] mb-4">Overview</h2>
                <p className="text-[#c9c9c9] leading-relaxed">
                    osu-mirror-rs is an open-source, community-driven project operated on a
                    best-effort basis. Because there is no centralized commercial operator,
                    the project maintainers do not assume responsibility for mediating or
                    arbitrating copyright disputes. As a result, DMCA procedures are not
                    implemented within this service.
                </p>
            </section>

            <section className="mb-8">
                <h2 className="text-xl font-semibold text-[#e0e0e0] mb-4">Project Policy</h2>
                <p className="text-[#c9c9c9] leading-relaxed">
                    The architecture and goals of the project prioritize transparency,
                    community autonomy, and minimal administrative overhead. Processing DMCA
                    notices requires operational capacity, legal oversight, and jurisdictional
                    structures that this project intentionally does not maintain. For this
                    reason, the maintainers have adopted a policy of not engaging with DMCA
                    submissions in any form.
                </p>
                <p className="text-[#c9c9c9] leading-relaxed mt-4">
                    You may submit a notice if you choose, but it will not be reviewed or
                    acted upon. This is not a dismissal of copyright concerns themselves,
                    but a reflection of the project's decentralized nature and its explicit
                    decision not to serve as an enforcement body.
                </p>
            </section>

            <section>
                <h2 className="text-xl font-semibold text-[#e0e0e0] mb-4">Response Time</h2>
                <p className="text-[#c9c9c9] leading-relaxed">
                    Since the project does not process DMCA notices, there is no response
                    timeline. No acknowledgements, evaluations, or removals will be issued.
                    All submissions of this type are disregarded in accordance with the
                    policy described above.
                </p>
            </section>
        </div>
    );
}
