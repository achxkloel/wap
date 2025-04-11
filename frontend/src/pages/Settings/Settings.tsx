import { useEffect, useState } from "react"
import { useNavigate } from "react-router-dom"
import Button from "@/components/Button"
import { environment } from "@/environment/environment"
import { logger } from "@/util/utils"

function Settings() {
    const navigate = useNavigate()

    const [theme, setTheme] = useState<"Light" | "Dark">("Light")
    const [notificationsEnabled, setNotificationsEnabled] = useState(true)
    const [radius, setRadius] = useState(50)
    const [loading, setLoading] = useState(true)
    const [error, setError] = useState("")

    const token = localStorage.getItem("auth_token")

    useEffect(() => {
        if (!token) {
            navigate("/login")
            return
        }

        const loadSettings = async () => {
            try {
                const res = await fetch(`${environment.baseUrl}/user/settings`, {
                    headers: {
                        Authorization: `Bearer ${token}`
                    }
                })

                if (!res.ok) throw new Error("Failed to load settings, please login")

                const data = await res.json()
                setTheme(data.theme)
                setNotificationsEnabled(data.notifications_enabled)
                setRadius(data.radius)
            } catch (err) {
                logger.error("Settings load error", err)
                setError("Failed to load settings")
            } finally {
                setLoading(false)
            }
        }

        loadSettings()
    }, [token, navigate])

    const handleSave = async () => {
        try {
            const body = JSON.stringify({
                theme,
                notifications_enabled: notificationsEnabled,
                radius
            });
            logger.debug("Settings body", body)
            const res = await fetch(`${environment.baseUrl}/user/settings`, {
                method: "PUT",
                headers: {
                    "Content-Type": "application/json",
                    Authorization: `Bearer ${token}`
                },
                body: body
            })

            if (!res.ok) {
                throw new Error("Failed to save settings")
            }

            logger.debug("Settings saved")
        } catch (err) {
            logger.error("Save failed", err)
            setError("Failed to save")
        }
    }

    if (!token) return null
    if (loading) return <div>Loading settings...</div>

    return (
        <div className="p-6 max-w-xl mx-auto space-y-6">
            <h1 className="text-2xl font-bold">Settings</h1>

            {error && <p className="text-red-500">{error}</p>}

            <div className="space-y-4">
                <div>
                    <label className="block font-medium mb-1">Theme</label>
                    <select
                        value={theme}
                        onChange={(e) => setTheme(e.target.value as "light" | "dark")}
                        className="border rounded p-2 w-full"
                    >
                        <option value="Light">Light</option>
                        <option value="Dark">Dark</option>
                    </select>
                </div>

                <div>
                    <label className="block font-medium mb-1">
                        Notifications Enabled
                    </label>
                    <input
                        type="checkbox"
                        checked={notificationsEnabled}
                        onChange={(e) => setNotificationsEnabled(e.target.checked)}
                    />
                </div>

                <div>
                    <label className="block font-medium mb-1">Radius</label>
                    <input
                        type="range"
                        min={0}
                        max={100}
                        value={radius}
                        onChange={(e) => setRadius(parseInt(e.target.value))}
                        className="w-full"
                    />
                    <div className="text-sm text-gray-600 mt-1">Value: {radius}</div>
                </div>

                <div className="flex justify-between pt-4">
                    <Button onClick={handleSave}>Save Settings</Button>
                    <Button
                        variant={theme}
                        onClick={() =>
                            setTheme((t) => (t === "light" ? "dark" : "light"))
                        }
                    >
                        Toggle Theme
                    </Button>
                </div>
            </div>
        </div>
    )
}

export default Settings
