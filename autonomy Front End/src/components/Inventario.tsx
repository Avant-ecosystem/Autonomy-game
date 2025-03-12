import { useState } from "react";

// Definir el tipo para los objetos del inventario
type Item = {
  id: number;
  nombre: string;
  tipo: string;
  icono: string;
};

const Inventario = () => {
  const [mostrarInventario, setMostrarInventario] = useState(false);

  // Datos de ejemplo para el inventario
  const inventario: Item[] = [
    { id: 1, nombre: "Poción de Salud", tipo: "Poción", icono: "/icons/pocion_salud.png" },
    { id: 2, nombre: "Espada de Acero", tipo: "Arma", icono: "/icons/espada.png" },
    { id: 3, nombre: "Armadura de Placas", tipo: "Armadura", icono: "/icons/armadura.png" },
  ];

  return (
    <div className="flex justify-center items-center">
      {/* Botón para abrir el inventario */}
      <button
        onClick={() => setMostrarInventario(!mostrarInventario)}
        className="bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded-lg shadow-md"
      >
        {mostrarInventario ? "Cerrar Inventario" : "Abrir Inventario"}
      </button>

      {/* Modal del inventario */}
      {mostrarInventario && (
        <div className="fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 bg-yellow-100 border-4 border-brown-800 rounded-2xl shadow-2xl p-8 text-center z-50 animate-fadeIn">
          <h2 className="text-2xl font-bold mb-4 text-brown-800">Inventario</h2>
          <ul className="grid grid-cols-3 gap-4">
            {inventario.map((item) => (
              <li key={item.id} className="flex flex-col items-center">
                <img src={item.icono} alt={item.nombre} className="w-16 h-16 mb-2" />
                <span className="font-semibold text-brown-800">{item.nombre}</span>
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
};

export default Inventario;