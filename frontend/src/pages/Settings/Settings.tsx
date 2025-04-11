import {
    Card,
    CardHeader,
    CardTitle,
    CardDescription,
    CardContent,
    CardFooter
} from "@/components/ui/card"
import { Label } from "@/components/ui/label"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Switch } from "@/components/ui/switch"
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue
} from "@/components/ui/select"
import { Slider } from "@/components/ui/slider"
import { useEffect, useState } from "react"
import { useNavigate } from "react-router-dom"
import { environment } from "@/environment/environment"
import { logger } from "@/util/utils"

function Settings() {
    const navigate = useNavigate()
    const token = localStorage.getItem("auth_token")

    const [theme, setTheme] = useState<"Light" | "Dark">("Light")
    const [notificationsEnabled, setNotificationsEnabled] = useState(true)
    const [radius, setRadius] = useState(50)
    const [loading, setLoading] = useState(true)
    const [error, setError] = useState("")

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
            })

            logger.debug("Settings body", body)

            const res = await fetch(`${environment.baseUrl}/user/settings`, {
                method: "PUT",
                headers: {
                    "Content-Type": "application/json",
                    Authorization: `Bearer ${token}`
                },
                body
            })

            if (!res.ok) throw new Error("Failed to save settings")

            logger.debug("Settings saved")
        } catch (err) {
            logger.error("Save failed", err)
            setError("Failed to save")
        }
    }

    if (!token) return null
    if (loading) return <div className="p-6 text-center">Loading settings...</div>

    return (
        <div className="max-w-xl mx-auto p-6">
            <Card>
                <CardHeader>
                    <CardTitle>Settings</CardTitle>
                    <CardDescription>Manage your account preferences</CardDescription>
                </CardHeader>
                <CardContent className="space-y-6">
                    {error && <p className="text-sm text-red-500">{error}</p>}

                    <div className="space-y-2">
                        <Label>Theme</Label>
                        <Select
                            value={theme}
                            onValueChange={(value: "Light" | "Dark") => setTheme(value)}
                        >
                            <SelectTrigger>
                                <SelectValue placeholder="Select theme" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="Light">Light</SelectItem>
                                <SelectItem value="Dark">Dark</SelectItem>
                            </SelectContent>
                        </Select>
                    </div>

                    <div className="flex items-center justify-between">
                        <Label>Notifications</Label>
                        <Switch
                            checked={notificationsEnabled}
                            onCheckedChange={setNotificationsEnabled}
                        />
                    </div>

                    <div>
                        <Label className="mb-1 block">Radius (km)</Label>
                        <Slider
                            min={0}
                            max={100}
                            step={1}
                            value={[radius]}
                            onValueChange={([val]) => setRadius(val)}
                        />
                        <div className="text-sm text-gray-500 mt-1">Current: {radius} km</div>
                    </div>
                </CardContent>
                <CardFooter className="flex justify-end">
                    <Button onClick={handleSave}>Save Settings</Button>
                </CardFooter>
            </Card>
        </div>
    )
}

export default Settings
