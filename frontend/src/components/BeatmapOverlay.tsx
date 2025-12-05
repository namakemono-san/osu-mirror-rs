import React, { useState } from "react";
import {
    X,
    Heart,
    Play,
    Download,
    Activity,
    Clock,
    Dot,
} from "lucide-react";

import { StandardGameModeIcon } from "@/components/icons/StandardGameModeIcon";
import { TaikoGameModeIcon } from "@/components/icons/TaikoGameModeIcon";
import { CatchGameModeIcon } from "@/components/icons/CatchGameModeIcon";
import { ManiaGameModeIcon } from "@/components/icons/ManiaGameModeIcon";

import { Button } from "@/components/ui/button";
import {
    Tooltip,
    TooltipTrigger,
    TooltipContent,
} from "@/components/ui/tooltip";

interface BeatmapOverlayProps {
    data: any;
    open: boolean;
    onClose: () => void;
}

function parseDate(d?: string | null) {
    if (!d) return "Unknown";
    const dt = new Date(d);
    if (isNaN(dt.getTime())) return "Unknown";
    return `${dt.getFullYear()}-${String(dt.getMonth() + 1).padStart(2, "0")}-${String(
        dt.getDate()
    ).padStart(2, "0")}`;
}

function formatLength(v?: number | null) {
    if (typeof v !== "number") return "Unknown";
    return `${Math.floor(v / 60)}:${String(v % 60).padStart(2, "0")}`;
}

function mapGenre(id?: number) {
    const g: Record<number, string> = {
        1: "Unspecified",
        2: "Video Game",
        3: "Anime",
        4: "Rock",
        5: "Pop",
        6: "Other",
        7: "Novelty",
        9: "Hip Hop",
        10: "Electronic",
    };
    return id && g[id] ? g[id] : "Unknown";
}

function mapLanguage(id?: number) {
    const g: Record<number, string> = {
        1: "Unspecified",
        2: "English",
        3: "Japanese",
        4: "Chinese",
        5: "Instrumental",
        6: "Korean",
        7: "French",
        8: "German",
        9: "Swedish",
        10: "Spanish",
        11: "Italian",
        12: "Russian",
        13: "Polish",
    };
    return id && g[id] ? g[id] : "Unknown";
}

function srToColor(sr: number): string {
    const points = [0.1, 1.25, 2, 2.5, 3.3, 4.2, 4.9, 5.8, 6.7, 7.7, 9];
    const colors = [
        "#4290FB",
        "#4FC0FF",
        "#4FFFD5",
        "#7CFF4F",
        "#F6F05C",
        "#FF8068",
        "#FF4E6F",
        "#C645B8",
        "#6563DE",
        "#18158E",
        "#000000",
    ];

    if (sr <= points[0]) return colors[0];
    if (sr >= points[points.length - 1]) return colors[colors.length - 1];

    for (let i = 0; i < points.length - 1; i++) {
        if (sr >= points[i] && sr < points[i + 1]) {
            return colors[i];
        }
    }
    return colors[0];
}

function modeIcon(mode: number) {
    switch (mode) {
        case 0: return <StandardGameModeIcon />;
        case 1: return <TaikoGameModeIcon />;
        case 2: return <CatchGameModeIcon />;
        case 3: return <ManiaGameModeIcon />;
        default: return <StandardGameModeIcon />;
    }
}

export default function BeatmapOverlay({ data, open, onClose }: BeatmapOverlayProps) {
    const [diffIndex, setDiffIndex] = useState(0);

    if (!open || !data) return null;

    const diffs = [...data.beatmaps].sort((a, b) =>
        (a.difficulty_rating ?? 0) - (b.difficulty_rating ?? 0)
    );

    const diff = diffs[diffIndex] ?? diffs[0];
    if (!diff) return null;

    const submitDate = parseDate(data.submitted_date);
    const rankedDate = parseDate(data.ranked_date);

    const bpm = diff.bpm ?? "Unknown";
    const totalLength = formatLength(diff.total_length);

    const maxCombo = diff.max_combo ?? "Unknown";
    const sliders = diff.count_sliders ?? "Unknown";
    const rating = data.rating ?? "N/A";
    const successRate = diff.passcount ?? 0;

    const genre = mapGenre(data.genre_id);
    const lang = mapLanguage(data.language_id);

    const tags =
        typeof data.tags === "string"
            ? data.tags.split(" ").filter(Boolean)
            : [];

    const hasVideo = data.video === true;

    return (
        <div
            className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-2 sm:p-4"
            onClick={onClose}
        >
            <div
                className="relative z-50 w-full max-w-[1150px] max-h-[95vh] bg-[#2d2d2d] text-[#f5f5f5] rounded-xl shadow-2xl flex flex-col overflow-hidden"
                onClick={(e) => e.stopPropagation()}
            >
                <div className="flex items-center justify-between px-4 py-2 bg-[#393a3b] border-b border-[#505457] shrink-0">
                    <span className="font-semibold text-[#f5f5f5]">Beatmap Info</span>
                    <button
                        className="p-1 text-[#c9c9c9] hover:text-white"
                        onClick={onClose}
                    >
                        <X className="h-5 w-5" />
                    </button>
                </div>

                <div className="flex-1 overflow-y-auto">
                    <div className="relative h-60 sm:h-[300px] lg:h-[360px] w-full">
                        <img
                            src={data.covers.cover}
                            alt=""
                            className="w-full h-full object-cover"
                        />
                        <div className="absolute inset-0 bg-linear-to-t from-black/95 via-black/60 to-transparent" />

                        <div className="absolute left-4 sm:left-6 top-2 z-20 max-w-[80%] lg:max-w-none">
                            <div className="inline-flex items-center bg-[#2d2d2d]/40 rounded-lg gap-0.5 p-1 backdrop-blur-sm shadow-lg">
                                {diffs.map((d: any, i: number) => {
                                    const sr = Number(d.difficulty_rating ?? 0);
                                    const color = srToColor(sr);
                                    const icon = modeIcon(d.mode_int);
                                    const isSelected = diffIndex === i;

                                    return (
                                        <Tooltip key={d.id}>
                                            <TooltipTrigger asChild>
                                                <button
                                                    onClick={(e) => {
                                                        e.stopPropagation();
                                                        setDiffIndex(i);
                                                    }}
                                                    type="button"
                                                    className={`
                                                        flex items-center gap-1 px-1.5 py-1.5 rounded-md transition-all
                                                        ${isSelected
                                                            ? "bg-[#505457]/70 shadow-md"
                                                            : "hover:bg-[#393a3b]/90"
                                                        }
                                                    `}
                                                >
                                                    <div
                                                        className="w-5 h-5 flex items-center justify-center"
                                                        style={{ color }}
                                                    >
                                                        {icon}
                                                    </div>
                                                </button>
                                            </TooltipTrigger>
                                            <TooltipContent>
                                                <div className="text-xs">
                                                    <div className="font-semibold">{d.version}</div>
                                                </div>
                                            </TooltipContent>
                                        </Tooltip>
                                    );
                                })}
                            </div>
                        </div>

                        <div className="absolute left-4 sm:left-7 top-12 sm:top-13 flex flex-col text-white drop-shadow">
                            <span className="text-sm sm:text-base font-semibold">{diff.version}</span>
                            <div className="flex items-center gap-2 text-xs sm:text-sm font-semibold">
                                <div className="flex items-center gap-1 text-yellow-300">
                                    <Play className="h-3 w-3 sm:h-4 sm:w-4" />
                                    <span className="text-white">{diff.playcount ?? 0}</span>
                                </div>
                                <div className="flex items-center gap-1 text-pink-300">
                                    <Heart className="h-3 w-3 sm:h-4 sm:w-4" />
                                    <span className="text-white">{data.favourite_count}</span>
                                </div>
                            </div>
                        </div>

                        <div className="absolute left-4 sm:left-6 bottom-3 sm:bottom-4 right-4 lg:right-[310px] flex flex-col text-white drop-shadow">
                            <h1 className="text-xl sm:text-2xl lg:text-4xl font-bold leading-tight truncate">
                                {data.title_unicode || data.title}
                            </h1>
                            <p className="text-sm sm:text-lg text-white/80 truncate">
                                {data.artist_unicode || data.artist}
                            </p>

                            <div className="flex items-center gap-2 mt-2 sm:mt-3">
                                <img
                                    src={`https://a.ppy.sh/${data.user_id}`}
                                    alt=""
                                    className="h-10 w-10 sm:h-14 sm:w-14 rounded-lg shrink-0"
                                />

                                <div className="min-w-0">
                                    <span className="text-xs sm:text-sm text-white/85 block truncate">
                                        mapped by{" "}
                                        <a
                                            href={`https://osu.ppy.sh/u/${data.user_id}`}
                                            className="font-semibold hover:underline"
                                        >
                                            {data.creator}
                                        </a>
                                    </span>
                                    <div className="flex flex-col text-[10px] sm:text-xs text-white/75 leading-tight">
                                        <span>Submitted: {submitDate}</span>
                                        <span>Ranked: {rankedDate}</span>
                                    </div>
                                </div>
                            </div>

                            <div className="flex flex-wrap gap-2 mt-2">
                                <DownloadButton
                                    label="Download"
                                    sub={!hasVideo ? undefined : "without video"}
                                    icon={<Download className="h-4 w-4" />}
                                />
                                {hasVideo && (
                                    <DownloadButton
                                        label="Download"
                                        sub="with video"
                                        icon={<Download className="h-4 w-4" />}
                                    />
                                )}
                                <DownloadButton
                                    label="osu!direct"
                                    icon={<Download className="h-4 w-4" />}
                                />
                            </div>
                        </div>

                        <div className="hidden lg:flex absolute right-6 top-[73%] -translate-y-1/2 w-72 flex-col gap-1">
                            <Button className="w-full rounded-xl bg-[#393a3b] border border-[#505457] hover:bg-[#505457] text-[#f5f5f5] shadow-sm flex items-center justify-center gap-2 py-3 text-sm">
                                <Play className="h-5 w-5" />
                            </Button>

                            <DifficultyStats diff={diff} bpm={bpm} totalLength={totalLength} maxCombo={maxCombo} sliders={sliders} />

                            <div className="rounded-xl bg-[#393a3b] border border-[#505457] px-4 py-3 shadow space-y-3 w-full">
                                <Diff
                                    label="User Rating"
                                    value={typeof rating === "number" ? rating.toFixed(2) : rating}
                                />
                                <Diff
                                    label="Success Rate"
                                    value={typeof successRate === "number" ? successRate.toFixed(2) : successRate}
                                />
                            </div>
                        </div>
                    </div>

                    <div className="lg:hidden p-4 space-y-2">
                        <Button className="w-full rounded-xl bg-[#393a3b] border border-[#505457] hover:bg-[#505457] text-[#f5f5f5] shadow-sm flex items-center justify-center gap-2 py-3 text-sm">
                            <Play className="h-5 w-5" />
                        </Button>

                        <DifficultyStats diff={diff} bpm={bpm} totalLength={totalLength} maxCombo={maxCombo} sliders={sliders} />

                        <div className="rounded-xl bg-[#393a3b] border border-[#505457] px-4 py-3 shadow space-y-3 w-full">
                            <Diff
                                label="User Rating"
                                value={typeof rating === "number" ? rating.toFixed(2) : rating}
                            />
                            <Diff
                                label="Success Rate"
                                value={typeof successRate === "number" ? successRate.toFixed(2) : successRate}
                            />
                        </div>
                    </div>

                    <div className="px-4 sm:px-6 pt-6 pb-6 grid grid-cols-1 md:grid-cols-2 gap-6 sm:gap-8 lg:w-[74%]">
                        <div className="flex flex-col gap-2 text-sm">
                            <h2 className="font-semibold text-[#f5f5f5]">Description</h2>
                            <div className="text-[#c9c9c9] leading-relaxed">
                                Sorry, but this design doesn't allow for an explanation.
                            </div>
                        </div>

                        <div className="flex flex-col gap-4 text-sm">
                            {data.source && data.source.trim() !== "" && (
                                <Meta title="Source" value={data.source} />
                            )}

                            <div className="flex gap-8">
                                <Meta title="Genre" value={genre} />
                                <Meta title="Language" value={lang} />
                            </div>

                            {tags.length > 0 && (
                                <div>
                                    <div className="text-xs font-semibold text-[#9e9e9e] mb-1">Tags</div>
                                    <div className="flex flex-wrap gap-1.5">
                                        {tags.map((t: string) => (
                                            <span
                                                key={t}
                                                className="px-2 py-0.5 rounded-full text-[11px] bg-[#505457] text-[#c9c9c9]"
                                            >
                                                {t}
                                            </span>
                                        ))}
                                    </div>
                                </div>
                            )}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

function DifficultyStats({ diff, bpm, totalLength, maxCombo, sliders }: any) {
    return (
        <div className="rounded-xl bg-[#393a3b] border border-[#505457] px-4 py-3 shadow space-y-3 w-full">
            <div className="grid grid-cols-4 gap-2">
                <Mini icon={<Activity className="h-4 w-4" />} val={bpm} label="BPM" />
                <Mini icon={<Clock className="h-4 w-4" />} val={totalLength} label="Length" />
                <Mini icon={<Dot className="h-4 w-4" />} val={maxCombo} label="Combo" />
                <Mini icon={<Dot className="h-4 w-4" />} val={sliders} label="Sliders" />
            </div>

            <div className="pt-2 border-t border-[#505457] space-y-2">
                <Diff label="HP Drain" value={diff.drain} />
                <Diff label="Accuracy" value={diff.accuracy} />
                <Diff label="Approach Rate" value={diff.ar} />
                <Diff label="Circle Size" value={diff.cs} />
                <Diff
                    label="Star Rating"
                    value={diff.difficulty_rating ? diff.difficulty_rating.toFixed(2) : "Unknown"}
                />
            </div>
        </div>
    );
}

function DownloadButton({
    label,
    sub,
    icon,
}: {
    label: string;
    sub?: string;
    icon: React.ReactNode;
}) {
    return (
        <Button
            className="bg-blue-600 hover:bg-blue-700 text-white h-9 sm:h-10 flex items-center justify-between gap-2 rounded-md px-3"
        >
            <div className="flex flex-col leading-tight text-left">
                <span className="text-sm">{label}</span>
                {sub && <span className="text-[10px] opacity-80">{sub}</span>}
            </div>
            {icon}
        </Button>
    );
}

function Mini({
    icon,
    val,
    label,
}: {
    icon: React.ReactNode;
    val: string | number;
    label: string;
}) {
    return (
        <Tooltip>
            <TooltipTrigger asChild>
                <div className="flex flex-row gap-1 items-center justify-center text-center bg-[#2d2d2d] rounded-md p-2 text-xs cursor-default">
                    <div className="text-[#9e9e9e]">{icon}</div>
                    <span className="font-medium text-[#f5f5f5]">{val}</span>
                </div>
            </TooltipTrigger>
            <TooltipContent>{label}</TooltipContent>
        </Tooltip>
    );
}

function Diff({ label, value }: { label: string; value: any }) {
    const num = Number(value);
    const ratio = Number.isNaN(num) ? 0 : Math.min(1, Math.max(0, num / 10));

    return (
        <div className="flex flex-col gap-1">
            <div className="flex justify-between text-[11px] text-[#9e9e9e]">
                <span>{label}</span>
                <span className="font-semibold text-[#f5f5f5]">{String(value)}</span>
            </div>
            <div className="h-1.5 bg-[#505457] rounded-full overflow-hidden">
                <div
                    className="h-full bg-blue-500"
                    style={{ width: `${ratio * 100}%` }}
                />
            </div>
        </div>
    );
}

function Meta({ title, value }: { title: string; value: string }) {
    return (
        <div className="flex flex-col text-sm">
            <div className="text-xs font-semibold text-[#9e9e9e]">{title}</div>
            <div className="text-[#f5f5f5]">{value}</div>
        </div>
    );
}