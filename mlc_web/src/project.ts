import './app.css'
import Project from './Project.svelte'

const app = new Project({
  target: document.getElementById('app') as Element,
})

export default app
