import { Outlet, Link } from "react-router-dom";
import { TooltipProvider } from "@/components/ui/tooltip";
import { API_BASE } from "@/config";

export default function MainLayout() {
    return (
        <TooltipProvider>
            <div className="flex min-h-screen flex-col bg-[#191919] text-[#f5f5f5]">
                <header className="bg-[#393a3b] border-b border-[#505457]">
                    <div className="mx-auto max-w-6xl px-6 py-2 flex justify-between items-center">
                        <Link to="/" className="text-xl font-semibold text-[#f5f5f5] hover:text-white">
                            osu-mirror-rs
                        </Link>

                        <nav className="space-x-5 text-base">
                            <Link to="/" className="text-[#c9c9c9] hover:text-white">
                                Beatmaps
                            </Link>
                            <a href={`${API_BASE}/docs`} target="_blank" rel="nofollow noreferrer" className="text-[#c9c9c9] hover:text-white">
                                Documents
                            </a>
                            <Link to="/about" className="text-[#c9c9c9] hover:text-white">
                                About
                            </Link>
                        </nav>
                    </div>
                </header>

                <main className="flex-1">
                    <Outlet />
                </main>

                <footer className="fixed bottom-0 left-0 w-full bg-[#393a3b] border-t border-[#505457] z-40">
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
                            <Link to="/donate" className="hover:text-white">Donate</Link>
                            <a href={`${API_BASE}/docs`} target="_blank" rel="nofollow noreferrer" className="hover:text-white">Documents</a>
                            <a href="https://github.com/namakemono-san/osu-mirror-rs" target="_blank" rel="nofollow noreferrer" className="hover:text-white">GitHub</a>
                            <Link to="/about" className="hover:text-white">About</Link>
                            <Link to="/status" className="hover:text-white">Status</Link>
                        </nav>
                    </div>
                </footer>
            </div>
        </TooltipProvider>
    );
}