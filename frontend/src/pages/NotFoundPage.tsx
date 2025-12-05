import { Link } from "react-router-dom";

export default function NotFoundPage() {
    return (
        <div className="mx-auto px-6 min-h-[calc(100vh-140px)] flex flex-col justify-center">
            <div className="flex flex-col items-center text-center select-none py-24">
                <h1 className="text-6xl font-bold text-[#e0e0e0] mb-4">404</h1>
                <h2 className="text-2xl font-semibold text-[#c9c9c9] mb-4">Page Not Found</h2>
                <p className="text-[#9e9e9e] text-lg leading-relaxed max-w-lg mb-8">
                    The page you're looking for doesn't exist or has been moved.
                </p>
                <Link
                    to="/"
                    className="px-6 py-3 rounded-md bg-blue-600 hover:bg-blue-700 text-white transition"
                >
                    Back to Home
                </Link>
            </div>
        </div>
    );
}