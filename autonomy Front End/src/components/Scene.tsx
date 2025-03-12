import React, { Suspense, useState } from "react";
import { Canvas } from "@react-three/fiber";
import { useGLTF } from "@react-three/drei";
import CardAvatar from "./CardAvatar";
import CardRoom from "./CardRoom";

const Scene = () => {
  const { scene, animations } = useGLTF("/avatar.glb");
  const { scene: sceneRoom, animations: animationsRoom } = useGLTF("/room.glb");
  const [chatMessage, setChatMessage] = useState("¡Hola, soy tu ser!");
  const [userInput, setUserInput] = useState("");
  const [loading, setLoading] = useState(false);

  const sendMessage = async () => {
    if (!userInput.trim()) return;
    setLoading(true);

    try {
      const response = await fetch("http://localhost:8080/chat/send-gemini", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ message: userInput, memory: "0" }),
      });

      const data = await response.json();
      setChatMessage(data.reply);
      setUserInput("");
    } catch (error) {
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className=" relative w-full h-[30rem]">
      {/* Escena 3D */}
      <Canvas>
        <ambientLight intensity={4} />
        <Suspense fallback={null}>
          <CardAvatar scene={scene} animations={animations} />
          <CardRoom scene={sceneRoom} animations={animationsRoom} />
        </Suspense>
      </Canvas>

      {/* 💬 Globo de Chat 2D */}
      <div
        className="absolute left-1/2 transform -translate-x-1/2 p-3 bg-white text-black rounded-lg shadow-lg"
        style={{
          top: "20%",
          maxWidth: "300px",
          textAlign: "center",
          fontSize: "16px",
          wordWrap: "break-word",
          whiteSpace: "pre-wrap",
        }}
      >
        {chatMessage}
      </div>

      {/* Interfaz de Chat */}
      <div className="absolute bottom-10 left-1/2 transform -translate-x-1/2 w-full max-w-lg text-center p-4 bg-white bg-opacity-70 rounded-lg z-10">
        <input
          type="text"
          value={userInput}
          onChange={(e) => setUserInput(e.target.value)}
          placeholder="Escribe un mensaje..."
          className="p-2 w-4/5 text-black rounded-md"
        />
        <button
          onClick={sendMessage}
          className="p-2 ml-2 bg-blue-500 text-white rounded-md"
        >
          Enviar
        </button>
        {loading && <div className="mt-2 text-white">Cargando...</div>}
        
      </div>
    </div>
  );
};

export default Scene;
