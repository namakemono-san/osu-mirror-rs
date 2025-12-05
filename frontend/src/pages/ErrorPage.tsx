import { useRouteError, isRouteErrorResponse, Link } from "react-router-dom";

export default function ErrorPage() {
    const error = useRouteError();

    let status = 500;
    let title = "Something went wrong";
    let message = "An unexpected error has occurred. Please try again later.";

    if (isRouteErrorResponse(error)) {
        status = error.status;
        switch (error.status) {
            case 404:
                title = "Page Not Found";
                message = "The page you're looking for doesn't exist or has been moved.";
                break;
            case 403:
                title = "Forbidden";
                message = "You don't have permission to access this page.";
                break;
            case 500:
                title = "Server Error";
                message = "Something went wrong on our end. Please try again later.";
                break;
            default:
                title = "Error";
                message = error.statusText || "An unexpected error occurred.";
        }
    } else if (error instanceof Error) {
        message = error.message;
    }

    return (
        <div className="flex min-h-screen flex-col bg-[#191919] text-[#f5f5f5]">
            <div className="mx-auto px-6 min-h-screen flex flex-col justify-center">
                <div className="flex flex-col items-center text-center select-none py-24">
                    <h1 className="text-6xl font-bold text-[#e0e0e0] mb-4">{status}</h1>
                    <h2 className="text-2xl font-semibold text-[#c9c9c9] mb-4">{title}</h2>
                    <p className="text-[#9e9e9e] text-lg leading-relaxed max-w-lg mb-8">
                        {message}
                    </p>
                    <div className="flex gap-4">
                        <button
                            onClick={() => window.location.reload()}
                            className="px-6 py-3 rounded-md bg-white/10 hover:bg-white/20 text-white transition"
                        >
                            Reload
                        </button>
                        <Link
                            to="/"
                            className="px-6 py-3 rounded-md bg-blue-600 hover:bg-blue-700 text-white transition"
                        >
                            Back to Home
                        </Link>
                    </div>
                </div>
            </div>
        </div>
    );
}