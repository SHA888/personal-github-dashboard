import { useEffect, useState } from 'react'

function App() {
  const [health, setHealth] = useState<'unknown' | 'ok' | 'down'>('unknown')

  useEffect(() => {
    const apiBase =
      (import.meta as any).env?.VITE_API_BASE_URL ||
      `${window.location.protocol}//${window.location.hostname}:3001`
    fetch(`${apiBase}/api/health`)
      .then((r) => (r.ok ? r.json() : Promise.reject()))
      .then(() => setHealth('ok'))
      .catch(() => setHealth('down'))
  }, [])

  return (
    <div className="min-h-screen bg-gray-50 text-gray-900">
      <div className="max-w-3xl mx-auto p-6">
        <h1 className="text-2xl font-bold mb-2">Personal GitHub Dashboard</h1>
        <p className="text-sm text-gray-600 mb-6">Vite + React + TypeScript + Tailwind</p>
        <div className="p-4 rounded border bg-white">
          <div className="font-medium">Backend health:</div>
          <div className="mt-1">
            {health === 'unknown' && <span className="text-gray-500">checking…</span>}
            {health === 'ok' && <span className="text-green-600">OK</span>}
            {health === 'down' && <span className="text-red-600">DOWN</span>}
          </div>
        </div>
      </div>
    </div>
  )
}

export default App
