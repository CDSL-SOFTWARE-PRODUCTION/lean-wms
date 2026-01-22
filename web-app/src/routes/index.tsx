import { createFileRoute } from '@tanstack/react-router'
import { supabase } from '../utils/supabase'

export const Route = createFileRoute('/')({
  loader: async () => {
    const { data: instruments } = await supabase.from('instruments').select()
    return { instruments }
  },
  component: Home,
})

function Home() {
  const { instruments } = Route.useLoaderData()

  return (
    <ul>
      {instruments?.map((instrument) => (
        <li key={instrument.name}>{instrument.name}</li>
      ))}
    </ul>
  )
}