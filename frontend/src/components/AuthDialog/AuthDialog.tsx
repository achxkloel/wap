import {Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger} from "@/components/ui/dialog"
import {Button} from "@/components/ui/button"
import {Input} from "@/components/ui/input"
import {Label} from "@/components/ui/label"
import {useState} from "react"
import {environment} from "@/environment/environment.ts";
import {logger} from "@/util/utils.ts";
import Cookies from "js-cookie"

export function AuthDialog() {
    const [mode, setMode] = useState<"login" | "register">("login")
    const [isOpen, setIsOpen] = useState(false)
    const [email, setEmail] = useState("")
    const [password, setPassword] = useState("")
    const [confirm, setConfirm] = useState("")
    const [error, setError] = useState("")
    const [loading, setLoading] = useState(false)

    const handleOpenChange = (open: boolean) => {
        setIsOpen(open)
        if (open) {
            setMode("login")
            setEmail("")
            setPassword("")
            setConfirm("")
            setError("")
        }
    }

    const handleSubmit = async () => {
        setError("")

        if (mode === "register" && password !== confirm) {
            setError("Passwords do not match")
            return
        }

        const endpoint =
            mode === "login"
                ? "/auth/login"
                : "/auth/register"

        const payload = {email, password}

        try {
            setLoading(true)
            const res = await fetch(environment.baseUrl + endpoint, {
                method: "POST",
                headers: {"Content-Type": "application/json"},
                body: JSON.stringify(payload)
            })

            const data = await res.json()

            const token = data.token;

            if (token) {
                localStorage.setItem("auth_token", token)
                logger.debug("Token stored:", token)
            }

            if (!res.ok) {
                setError(data?.message || "Something went wrong")
                return
            }

            // success handling
            console.log("✅ Auth success:", data)
            setIsOpen(false)
        } catch (err) {
            console.error(err)
            setError("Something went wrong")
        } finally {
            setLoading(false)
        }

        // print token that is now set on cookies in token key
        const token = Cookies.get("token") // TODO: fix it
        logger.debug("Token:", token)
    }

    return (
        <Dialog open={isOpen} onOpenChange={handleOpenChange}>
            <DialogTrigger asChild>
                <Button variant="outline">Login</Button>
            </DialogTrigger>
            <DialogContent className="sm:max-w-[425px]">
                <DialogHeader>
                    <DialogTitle>
                        {mode === "login" ? "Login" : "Register"}
                    </DialogTitle>
                    <DialogDescription>
                        {mode === "login"
                            ? "Login to access your account"
                            : "Create a new account"}
                    </DialogDescription>
                </DialogHeader>

                <form
                    className="grid gap-4 py-4"
                    onSubmit={(e) => {
                        e.preventDefault()
                        handleSubmit()
                    }}
                >
                    <div className="grid gap-2">
                        <Label htmlFor="email">Email</Label>
                        <Input
                            id="email"
                            type="email"
                            value={email}
                            onChange={(e) => setEmail(e.target.value)}
                            required
                        />
                    </div>
                    <div className="grid gap-2">
                        <Label htmlFor="password">Password</Label>
                        <Input
                            id="password"
                            type="password"
                            value={password}
                            onChange={(e) => setPassword(e.target.value)}
                            required
                        />
                    </div>
                    {mode === "register" && (
                        <div className="grid gap-2">
                            <Label htmlFor="confirm">Confirm Password</Label>
                            <Input
                                id="confirm"
                                type="password"
                                value={confirm}
                                onChange={(e) => setConfirm(e.target.value)}
                                required
                            />
                        </div>
                    )}

                    {error && (
                        <p className="text-sm text-red-500 -mt-2">{error}</p>
                    )}
                </form>

                <DialogFooter className="flex justify-between">
                    <Button
                        variant="ghost"
                        type="button"
                        onClick={() =>
                            setMode(mode === "login" ? "register" : "login")
                        }
                    >
                        {mode === "login" ? "Switch to Register" : "Switch to Login"}
                    </Button>
                    <Button type="submit" onClick={handleSubmit} disabled={loading}>
                        {loading
                            ? "Loading..."
                            : mode === "login"
                                ? "Login"
                                : "Register"}
                    </Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    )
}
