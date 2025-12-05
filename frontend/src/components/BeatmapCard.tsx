import { useState, memo } from "react";

import {
    Tooltip,
    TooltipTrigger,
    TooltipContent,
} from "@/components/ui/tooltip";

import { Button } from "@/components/ui/button";

import {
    Video,
    ImageIcon,
    Heart,
    Play,
    CalendarDays,
    Link,
    Download,
    Globe,
    Check,
    FileDown,
} from "lucide-react";

import { StandardGameModeIcon } from "@/components/icons/StandardGameModeIcon";
import { TaikoGameModeIcon } from "@/components/icons/TaikoGameModeIcon";
import { CatchGameModeIcon } from "@/components/icons/CatchGameModeIcon";
import { ManiaGameModeIcon } from "@/components/icons/ManiaGameModeIcon";

export interface BeatmapCovers {
    cover: string;
    "cover@2x"?: string;
    card?: string;
    "card@2x"?: string;
    list?: string;
    "list@2x"?: string;
    slimcover?: string;
    "slimcover@2x"?: string;
}

export interface Beatmap {
    id: number;
    mode_int: number;
    difficulty_rating: number;
    version: string;
}

export interface BeatmapSet {
    id: number;
    title: string;
    artist: string;
    creator: string;
    user_id: number;
    status: string;
    covers: BeatmapCovers;
    beatmaps?: Beatmap[];
    favourite_count: number;
    play_count: number;
    ranked_date?: string | null;
    video?: boolean;
    storyboard?: boolean;
}

export interface BeatmapCardProps {
    data: BeatmapSet;
    onClick?: () => void;
}

const CARD_BG = "bg-[#2d2d2d]";
const CARD_LINE = "bg-[#505457]";
const CARD_BG_DIFF = "bg-[#393a3b]";
const TEXT_MAIN = "text-[#f5f5f5]";
const TEXT_SUB = "text-[#c9c9c9]";

const API_HOST = typeof window !== "undefined" ? window.location.origin : "";

const statusMap: Record<string, { label: string; color: string }> = {
    ranked: { label: "RANKED", color: "bg-green-600 text-white" },
    approved: { label: "APPROVED", color: "bg-blue-600 text-white" },
    qualified: { label: "QUALIFIED", color: "bg-yellow-600 text-neutral-900" },
    loved: { label: "LOVED", color: "bg-pink-600 text-white" },
    pending: { label: "PENDING", color: "bg-neutral-400 text-neutral-900" },
    graveyard: { label: "GRAVEYARD", color: "bg-neutral-600 text-white" },
};

const SR_POINTS = [0.1, 1.25, 2, 2.5, 3.3, 4.2, 4.9, 5.8, 6.7, 7.7, 9];
const SR_COLORS = [
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

function srToColor(sr: number): string {
    if (sr <= SR_POINTS[0]) return SR_COLORS[0];
    if (sr >= SR_POINTS[SR_POINTS.length - 1]) return SR_COLORS[SR_COLORS.length - 1];

    for (let i = 0; i < SR_POINTS.length - 1; i++) {
        if (sr >= SR_POINTS[i] && sr < SR_POINTS[i + 1]) {
            return SR_COLORS[i];
        }
    }
    return SR_COLORS[0];
}

function parseDate(dt?: string | null): string {
    if (!dt) return "Unknown";
    const d = new Date(dt);
    if (isNaN(d.getTime())) return "Unknown";
    return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
}

function formatFullDate(dt?: string | null): string {
    if (!dt) return "Unknown";

    const cleaned = dt.match(/[0-9TZ:\-]+/)?.[0] ?? dt;
    const d = new Date(cleaned);
    if (isNaN(d.getTime())) return dt;

    return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")} ${String(d.getHours()).padStart(2, "0")}:${String(d.getMinutes()).padStart(2, "0")}:${String(d.getSeconds()).padStart(2, "0")}`;
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

interface IconButtonProps {
    icon: React.ReactNode;
    label: string;
    onClick?: (e: React.MouseEvent<HTMLButtonElement>) => void;
    copyUrl?: string;
}

const IconButton = memo(function IconButton({ icon, label, onClick, copyUrl }: IconButtonProps) {
    const [copied, setCopied] = useState(false);

    const handleClick = (e: React.MouseEvent<HTMLButtonElement>) => {
        e.stopPropagation();

        if (copyUrl) {
            navigator.clipboard.writeText(copyUrl);
            setCopied(true);
            setTimeout(() => setCopied(false), 800);
            return;
        }

        onClick?.(e);
    };

    return (
        <Tooltip>
            <TooltipTrigger asChild>
                <Button
                    onClick={handleClick}
                    className="flex w-12 h-7 items-center justify-center rounded-md bg-[#393a3b] hover:bg-[#505457] text-white p-0"
                >
                    {copied ? <Check className="h-4 w-4 text-green-400" /> : icon}
                </Button>
            </TooltipTrigger>
            <TooltipContent className="text-white">
                {label}
            </TooltipContent>
        </Tooltip>
    );
});

export default function BeatmapCard({ data, onClick }: BeatmapCardProps) {
    const status = statusMap[data.status] ?? {
        label: "UNKNOWN",
        color: "bg-neutral-500 text-white",
    };

    const diffs = data.beatmaps
        ? [...data.beatmaps].sort((a, b) => (a.difficulty_rating ?? 0) - (b.difficulty_rating ?? 0))
        : [];

    return (
        <div
            onClick={onClick}
            className={`${CARD_BG} rounded-2xl overflow-hidden shadow-sm cursor-pointer transition-all duration-200 hover:scale-[1.02]`}
        >
            <div className="relative h-36 sm:h-40 w-full overflow-hidden">
                <img
                    src={data.covers.cover}
                    alt=""
                    className="h-full w-full object-cover"
                />

                <div className="absolute inset-0 bg-linear-to-t from-black/60 via-black/20 to-transparent" />

                <div className="absolute flex left-3 sm:left-4 top-2.5 sm:top-3 gap-1">
                    <span className={`rounded-lg px-2 sm:px-3 py-0.5 text-xs sm:text-base font-bold ${status.color}`}>
                        {status.label}
                    </span>

                    {data.video && (
                        <Tooltip>
                            <TooltipTrigger asChild>
                                <span className="rounded-md px-2 py-1 bg-purple-700 text-white flex items-center shadow-sm">
                                    <Video className="h-4 w-4" />
                                </span>
                            </TooltipTrigger>
                            <TooltipContent>This beatmap contains video</TooltipContent>
                        </Tooltip>
                    )}

                    {data.storyboard && (
                        <Tooltip>
                            <TooltipTrigger asChild>
                                <span className="rounded-md px-2 py-1 bg-blue-700 text-white flex items-center shadow-sm">
                                    <ImageIcon className="h-4 w-4" />
                                </span>
                            </TooltipTrigger>
                            <TooltipContent>This beatmap contains storyboard</TooltipContent>
                        </Tooltip>
                    )}
                </div>

                <div className="absolute bottom-3 left-4 right-4">
                    <h2 className="text-2xl font-bold text-white drop-shadow truncate">
                        {data.title}
                    </h2>
                    <p className="text-sm text-gray-200 drop-shadow truncate">
                        {data.artist}
                    </p>
                </div>
            </div>

            <div className="px-4 py-2 flex flex-col sm:flex-row sm:items-center sm:justify-between gap-1 text-sm">
                <p className={`${TEXT_SUB} truncate`}>
                    mapped by{" "}
                    <a
                        href={`https://osu.ppy.sh/users/${data.user_id}`}
                        className="font-semibold text-pink-500 hover:underline"
                        onClick={(e) => e.stopPropagation()}
                    >
                        {data.creator}
                    </a>
                </p>

                <div className="flex items-center gap-2">
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <div className="flex items-center gap-1 rounded-md bg-[rgba(255,255,255,0.1)] px-2 py-1 cursor-default">
                                <Heart className="h-4 w-4 text-pink-400" />
                                <span className={TEXT_MAIN}>{Number(data.favourite_count).toLocaleString()}</span>
                            </div>
                        </TooltipTrigger>
                        <TooltipContent>
                            Favourites Count: {Number(data.favourite_count).toLocaleString()}
                        </TooltipContent>
                    </Tooltip>

                    <Tooltip>
                        <TooltipTrigger asChild>
                            <div className="flex items-center gap-1 rounded-md bg-[rgba(255,255,255,0.1)] px-2 py-1 cursor-default">
                                <Play className="h-4 w-4 text-yellow-400" />
                                <span className={TEXT_MAIN}>{Number(data.play_count).toLocaleString()}</span>
                            </div>
                        </TooltipTrigger>
                        <TooltipContent>
                            Play Count: {Number(data.play_count).toLocaleString()}
                        </TooltipContent>
                    </Tooltip>

                    <Tooltip>
                        <TooltipTrigger asChild>
                            <div className="flex items-center gap-1 rounded-md bg-[rgba(255,255,255,0.1)] px-2 py-1 cursor-default">
                                <CalendarDays className="h-4 w-4 text-blue-400" />
                                <span className={TEXT_MAIN}>{parseDate(data.ranked_date)}</span>
                            </div>
                        </TooltipTrigger>
                        <TooltipContent>
                            {formatFullDate(data.ranked_date) ?? "Unknown"}
                        </TooltipContent>
                    </Tooltip>
                </div>
            </div>

            <div className={`w-full h-0.5 ${CARD_LINE}`} />

            <div className={`px-4 py-2 flex items-center gap-2 overflow-x-auto ${CARD_BG_DIFF}`}>
                {diffs.length > 0 ? diffs.map((diff) => {
                    const sr = Number(diff.difficulty_rating ?? 0);

                    return (
                        <Tooltip key={diff.id}>
                            <TooltipTrigger asChild>
                                <div
                                    className="h-6 w-6 rounded-full flex items-center justify-center"
                                    style={{ color: srToColor(sr) }}
                                >
                                    {modeIcon(diff.mode_int)}
                                </div>
                            </TooltipTrigger>
                            <TooltipContent>
                                <div className="text-xs">
                                    <div className="font-semibold">{diff.version}</div>
                                    <div className="text-neutral-400">★ {sr.toFixed(2)}</div>
                                </div>
                            </TooltipContent>
                        </Tooltip>
                    );
                }) : (
                    <>
                        <span className="h-4 w-4 rounded-full bg-neutral-500" />
                        <span className="h-4 w-4 rounded-full bg-neutral-500" />
                        <span className="h-4 w-4 rounded-full bg-neutral-500" />
                    </>
                )}
            </div>

            <div className={`w-full h-0.5 ${CARD_LINE}`} />

            <div className="px-4 py-2 flex justify-end gap-2">
                <IconButton
                    icon={<Link className="h-4 w-4" />}
                    label="Copy Download URL"
                    copyUrl={`${API_HOST}/d/${data.id}`}
                />

                <IconButton
                    icon={<ImageIcon className="h-4 w-4" />}
                    label="Download Background"
                    onClick={() => {
                        window.open(data.covers["cover@2x"] || data.covers.cover, "_blank");
                    }}
                />

                <IconButton
                    icon={<Download className="h-4 w-4" />}
                    label="Download Beatmapsets"
                    onClick={() => {
                        window.open(`${API_HOST}/d/${data.id}`, "_blank");
                    }}
                />

                <IconButton
                    icon={<Globe className="h-4 w-4" />}
                    label="osu!direct"
                    onClick={() => {
                        window.location.href = `osu://dl/${data.id}`;
                    }}
                />

                <IconButton
                    icon={<FileDown className="h-4 w-4" />}
                    label="Go to beatmap page"
                    onClick={() => {
                        window.open(`https://osu.ppy.sh/beatmapsets/${data.id}`, "_blank");
                    }}
                />
            </div>
        </div>
    );
}