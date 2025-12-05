"use client";

import { useState, useEffect } from "react";
import { TooltipProvider } from "@/components/ui/tooltip";
import BeatmapCard, { type BeatmapSet } from "@/components/BeatmapCard";
import BeatmapOverlay from "@/components/BeatmapOverlay";
import { API_BASE } from "../config"

export default function PageLayout() {
    const [list, setList] = useState<BeatmapSet[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    const [open, setOpen] = useState(false);
    const [selected, setSelected] = useState<BeatmapSet | null>(null);

    const openOverlay = (data: BeatmapSet) => {
        setSelected(data);
        setOpen(true);
    };

    const closeOverlay = () => {
        setOpen(false);
        setSelected(null);
    };

    useEffect(() => {
        async function load() {
            try {
                const res = await fetch(`${API_BASE}/v2/search`);

                if (!res.ok) {
                    throw new Error(`HTTP Error: ${res.status}`);
                }

                const json = await res.json();
                setList(json.beatmapsets);
            } catch (e: any) {
                console.error("API error:", e);
                setError(e?.message ?? "Unknown error");
            } finally {
                setLoading(false);
            }
        }

        load();
    }, []);

    return (
        <TooltipProvider>
            <div className="flex min-h-screen flex-col bg-[#191919] text-[#f5f5f5]">
                <header className="bg-[#393a3b] border-b border-[#505457]">
                    <div className="mx-auto max-w-6xl px-6 py-2 flex justify-between items-center">
                        <h1 className="text-xl font-semibold text-[#f5f5f5]">osu-mirror-rs</h1>

                        <nav className="space-x-5 text-base">
                            <a className="text-[#c9c9c9] hover:text-white cursor-pointer">Beatmaps</a>
                            <a className="text-[#c9c9c9] hover:text-white cursor-pointer">Documents</a>
                            <a className="text-[#c9c9c9] hover:text-white cursor-pointer">About</a>
                        </nav>
                    </div>
                </header>

                <main className="flex-1">
                    <div className="mx-auto px-6 min-h-[calc(100vh-140px)] flex flex-col justify-center">
                        {loading && (
                            <div className="flex flex-col items-center text-center select-none py-24">
                                <h2 className="text-4xl font-bold text-[#e0e0e0] mb-2">Loading…</h2>
                                <p className="text-[#c9c9c9] text-lg">
                                    Fetching beatmaps from the server.
                                </p>
                            </div>
                        )}

                        {error && (
                            <div className="flex flex-col items-center text-center select-none py-24">

                                <h1 className="text-4xl font-bold text-[#e0e0e0] mb-2">
                                    Oops…
                                </h1>

                                <p className="text-[#c9c9c9] text-lg leading-relaxed max-w-lg mb-4">
                                    We were unable to fetch data from the server.<br />
                                    The API may be temporarily unavailable.<br />
                                    Please try again in a moment.
                                </p>

                                <div className="flex gap-4">
                                    <button
                                        onClick={() => window.location.reload()}
                                        className="px-4 py-2 rounded-md bg-white/10 hover:bg-white/20 text-white transition"
                                    >
                                        Reload
                                    </button>
                                </div>
                            </div>
                        )}

                        {!loading && !error && list.length === 0 && (
                            <div className="flex flex-col items-center text-center select-none py-24">
                                <h2 className="text-4xl font-bold text-[#e0e0e0] mb-2">No Results</h2>
                                <p className="text-[#c9c9c9] text-lg">
                                    No beatmaps were found for this query.
                                </p>
                            </div>
                        )}

                        {!loading && !error && list.length > 0 && (
                            <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 pt-4 pb-24">
                                {list.map((bm) => (
                                    <BeatmapCard
                                        key={bm.id}
                                        data={bm}
                                        onClick={() => openOverlay(bm)}
                                    />
                                ))}
                            </div>
                        )}
                    </div>
                </main>

                <footer className="fixed bottom-0 left-0 w-full bg-[#393a3b] border-t border-[#505457] z-50">
                    <div className="w-full px-6 py-4 flex flex-col sm:flex-row items-center justify-between text-sm">
                        <div className="flex items-center gap-2 text-[#c9c9c9] whitespace-nowrap">
                            <span className="flex items-center gap-1">
                                Made with
                                <span className="mx-0.5 text-2xs opacity-80 saturate-0 hover:saturate-100 cursor-pointer">❣️</span>
                                by{" "}
                                <a
                                    href="https://github.com/namakemono-san"
                                    target="_blank"
                                    rel="nofollow noreferrer"
                                    className="hover:text-[#f5f5f5] font-bold"
                                >
                                    namakemono-san
                                </a>
                            </span>

                            <span className="text-[#505457]">|</span>

                            <span>
                                Powered by{" "}
                                <a
                                    href="https://github.com/namakemono-san/osu-mirror-rs"
                                    target="_blank"
                                    rel="nofollow noreferrer"
                                    className="hover:text-[#f5f5f5] font-bold"
                                >
                                    osu-mirror-rs
                                </a>
                            </span>
                        </div>

                        <div className="text-[#9e9e9e] text-xs py-2 sm:py-0 whitespace-nowrap">
                            This project is not affiliated with osu! or ppy.
                        </div>

                        <nav className="flex flex-wrap justify-center sm:justify-end gap-4 text-[#c9c9c9] whitespace-nowrap">
                            <a className="hover:text-white cursor-pointer">Donate</a>
                            <a className="hover:text-white cursor-pointer">DMCA</a>
                            <a className="hover:text-white cursor-pointer">Documents</a>
                            <a className="hover:text-white cursor-pointer">GitHub</a>
                            <a className="hover:text-white cursor-pointer">About</a>
                            <a className="hover:text-white cursor-pointer">Status</a>
                        </nav>
                    </div>
                </footer>

                <BeatmapOverlay open={open} onClose={closeOverlay} data={selected} />
            </div>
        </TooltipProvider>
    );
}