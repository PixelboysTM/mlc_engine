import './app.css'
import Viewer3D from './Viewer3D.svelte'

const app = new Viewer3D({
  target: document.getElementById('app') as Element,
})

export default app
