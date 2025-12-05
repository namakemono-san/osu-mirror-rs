import { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import BeatmapCard, { type BeatmapSet } from "@/components/BeatmapCard";
import BeatmapOverlay from "@/components/BeatmapOverlay";
import { API_BASE } from "../config";

export default function BeatmapsPage() {
    const { id } = useParams<{ id: string }>();
    const navigate = useNavigate();

    const [list, setList] = useState<BeatmapSet[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    const [selected, setSelected] = useState<BeatmapSet | null>(null);
    const [modalLoading, setModalLoading] = useState(false);

    const [query, setQuery] = useState("");

    async function fetchBeatmaps(searchQuery = "") {
        setLoading(true);
        setError(null);

        try {
            const url = searchQuery
                ? `${API_BASE}/v2/search?q=${encodeURIComponent(searchQuery)}`
                : `${API_BASE}/v2/search`;

            const res = await fetch(url);
            if (!res.ok) throw new Error(`HTTP Error: ${res.status}`);

            const json = await res.json();
            setList(json.beatmapsets);
        } catch (e: any) {
            console.error("API error:", e);
            setError(e?.message ?? "Unknown error");
        } finally {
            setLoading(false);
        }
    }

    useEffect(() => {
        fetchBeatmaps();
    }, []);

    useEffect(() => {
        if (!id) {
            setSelected(null);
            return;
        }

        const numId = Number(id);

        const found = list.find((bm) => bm.id === numId);
        if (found) {
            setSelected(found);
            return;
        }

        if (!loading) {
            setModalLoading(true);
            fetch(`${API_BASE}/v2/beatmapsets/${id}`)
                .then((res) => {
                    if (!res.ok) throw new Error(`HTTP Error: ${res.status}`);
                    return res.json();
                })
                .then((data) => {
                    if (data) {
                        setSelected(data);
                    } else {
                        navigate("/", { replace: true });
                    }
                })
                .catch((e) => {
                    console.error("Failed to fetch beatmapset:", e);
                    navigate("/", { replace: true });
                })
                .finally(() => setModalLoading(false));
        }
    }, [id, list, loading, navigate]);

    const openOverlay = (data: BeatmapSet) => {
        navigate(`/beatmapsets/${data.id}`);
    };

    const closeOverlay = () => {
        navigate("/");
    };

    const onSearch = (e: React.FormEvent) => {
        e.preventDefault();
        fetchBeatmaps(query);
    };

    return (
        <>
            <div className="mx-auto px-6 min-h-[calc(100vh-140px)] flex flex-col justify-center">

                <form onSubmit={onSearch} className="mt-6 mb-4 flex justify-center">
                    <input
                        type="text"
                        value={query}
                        onChange={(e) => setQuery(e.target.value)}
                        placeholder="Search beatmaps…"
                        className="px-4 py-2 w-full max-w-md rounded-md bg-white/10 text-white placeholder-gray-400 focus:outline-none mr-4"
                    />
                    <button
                        type="submit"
                        className="px-4 py-2 rounded-md bg-blue-500 hover:bg-blue-600 transition text-white"
                    >
                        Search
                    </button>
                </form>

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
                        <h1 className="text-4xl font-bold text-[#e0e0e0] mb-2">Oops…</h1>
                        <p className="text-[#c9c9c9] text-lg leading-relaxed max-w-lg mb-4">
                            We were unable to fetch data from the server.<br />
                            The API may be temporarily unavailable.<br />
                            Please try again in a moment.
                        </p>
                        <div className="flex gap-4">
                            <button
                                onClick={() => fetchBeatmaps(query)}
                                className="px-4 py-2 rounded-md bg-white/10 hover:bg-white/20 text-white transition"
                            >
                                Retry
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

            {modalLoading && (
                <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm">
                    <div className="text-white text-xl">Loading…</div>
                </div>
            )}

            <BeatmapOverlay
                open={!!selected && !modalLoading}
                onClose={closeOverlay}
                data={selected}
            />
        </>
    );
}
