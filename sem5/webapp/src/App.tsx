import { useState } from "react";
import flappyImg from "./assets/flappybird.png";
import "./App.css";

function App() {
  //const [count, setCount] = useState(0);

  function App() {
    return (
      <>
        <section id="center">
          <div className="flappy">
            <img
              src={flappyImg}
              className="base"
              width="auto"
              height="170px"
              alt="a yellow pixelated bird"
            />
          </div>
          <div></div>
        </section>
      </>
    );
  }
}

export default App;
