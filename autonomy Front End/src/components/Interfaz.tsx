// components/Interfaz.js
'use client'
import dynamic from 'next/dynamic';
import Inventario from './Inventario';

const Scene = dynamic(() => import('../components/Scene'), {
  ssr: false,
});


const Interfaz = () => {
  return (
    <>
      <div className="relative flex size-full min-h-screen flex-col bg-[#171122] dark group/design-root overflow-x-hidden" style={{ fontFamily: '"Space Grotesk", "Noto Sans", sans-serif' }}>
        <div className="layout-container flex h-full grow flex-col">
          <header className="flex items-center justify-between whitespace-nowrap border-b border-solid border-b-[#302447] px-10 py-3">
            <div className="flex items-center gap-4 text-white">
              <div className="size-4">
                <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <g clipPath="url(#clip0_6_330)">
                    <path
                      fillRule="evenodd"
                      clipRule="evenodd"
                      d="M24 0.757355L47.2426 24L24 47.2426L0.757355 24L24 0.757355ZM21 35.7574V12.2426L9.24264 24L21 35.7574Z"
                      fill="currentColor"
                    ></path>
                  </g>
                  <defs>
                    <clipPath id="clip0_6_330"><rect width="48" height="48" fill="white"></rect></clipPath>
                  </defs>
                </svg>
              </div>
              <h2 className="text-white text-lg font-bold leading-tight tracking-[-0.015em]">Autonomy</h2>
            </div>
            <div className="flex flex-1 justify-end gap-8">
              <div className="flex items-center gap-9">
                <a className="text-white text-sm font-medium leading-normal" href="#">Dashboard</a>
                <a className="text-white text-sm font-medium leading-normal" href="#">Marketplace</a>
                <a className="text-white text-sm font-medium leading-normal" href="#">Help</a>
              </div>
              <button
                className="flex max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-full h-10 bg-[#302447] text-white gap-2 text-sm font-bold leading-normal tracking-[0.015em] min-w-0 px-2.5"
              >
                <div className="text-white" data-icon="Gear" data-size="20px" data-weight="regular">
                  <svg xmlns="http://www.w3.org/2000/svg" width="20px" height="20px" fill="currentColor" viewBox="0 0 256 256">
                    <path
                      d="M128,80a48,48,0,1,0,48,48A48.05,48.05,0,0,0,128,80Zm0,80a32,32,0,1,1,32-32A32,32,0,0,1,128,160Zm88-29.84q.06-2.16,0-4.32l14.92-18.64a8,8,0,0,0,1.48-7.06,107.21,107.21,0,0,0-10.88-26.25,8,8,0,0,0-6-3.93l-23.72-2.64q-1.48-1.56-3-3L186,40.54a8,8,0,0,0-3.94-6,107.71,107.71,0,0,0-26.25-10.87,8,8,0,0,0-7.06,1.49L130.16,40Q128,40,125.84,40L107.2,25.11a8,8,0,0,0-7.06-1.48A107.6,107.6,0,0,0,73.89,34.51a8,8,0,0,0-3.93,6L67.32,64.27q-1.56,1.49-3,3L40.54,70a8,8,0,0,0-6,3.94,107.71,107.71,0,0,0-10.87,26.25,8,8,0,0,0,1.49,7.06L40,125.84Q40,128,40,130.16L25.11,148.8a8,8,0,0,0-1.48,7.06,107.21,107.21,0,0,0,10.88,26.25,8,8,0,0,0,6,3.93l23.72,2.64q1.49,1.56,3,3L70,215.46a8,8,0,0,0,3.94,6,107.71,107.71,0,0,0,26.25,10.87,8,8,0,0,0,7.06-1.49L125.84,216q2.16.06,4.32,0l18.64,14.92a8,8,0,0,0,7.06,1.48,107.21,107.21,0,0,0,26.25-10.88,8,8,0,0,0,3.93-6l2.64-23.72q1.56-1.48,3-3L215.46,186a8,8,0,0,0,6-3.94,107.71,107.71,0,0,0,10.87-26.25,8,8,0,0,0-1.49-7.06Zm-16.1-6.5a73.93,73.93,0,0,1,0,8.68,8,8,0,0,0,1.74,5.48l14.19,17.73a91.57,91.57,0,0,1-6.23,15L187,173.11a8,8,0,0,0-5.1,2.64,74.11,74.11,0,0,1-6.14,6.14,8,8,0,0,0-2.64,5.1l-2.51,22.58a91.32,91.32,0,0,1-15,6.23l-17.74-14.19a8,8,0,0,0-5-1.75h-.48a73.93,73.93,0,0,1-8.68,0,8,8,0,0,0-5.48,1.74L100.45,215.8a91.57,91.57,0,0,1-15-6.23L82.89,187a8,8,0,0,0-2.64-5.1,74.11,74.11,0,0,1-6.14-6.14,8,8,0,0,0-5.1-2.64L46.43,170.6a91.32,91.32,0,0,1-6.23-15l14.19-17.74a8,8,0,0,0,1.74-5.48,73.93,73.93,0,0,1,0-8.68,8,8,0,0,0-1.74-5.48L40.2,100.45a91.57,91.57,0,0,1,6.23-15L69,82.89a8,8,0,0,0,5.1-2.64,74.11,74.11,0,0,1,6.14-6.14A8,8,0,0,0,82.89,69L85.4,46.43a91.32,91.32,0,0,1,15-6.23l17.74,14.19a8,8,0,0,0,5.48,1.74,73.93,73.93,0,0,1,8.68,0,8,8,0,0,0,5.48-1.74L155.55,40.2a91.57,91.57,0,0,1,15,6.23L173.11,69a8,8,0,0,0,2.64,5.1,74.11,74.11,0,0,1,6.14,6.14,8,8,0,0,0,5.1,2.64l22.58,2.51a91.32,91.32,0,0,1,6.23,15l-14.19,17.74A8,8,0,0,0,199.87,123.66Z"
                    ></path>
                  </svg>
                </div>
              </button>
              <div
                className="bg-center bg-no-repeat aspect-square bg-cover rounded-full size-10"
                style={{ backgroundImage: 'url("https://cdn.usegalileo.ai/sdxl10/7867c71d-8f0b-46f3-9727-bc546c373b51.png")' }}
              ></div>
            </div>
          </header>
          <div className="px-40 flex flex-1 justify-center py-5">
            <div className="layout-content-container flex flex-col max-w-[960px] flex-1">
              <h3 className="text-white tracking-light text-2xl font-bold leading-tight px-4 text-left pb-2 pt-5">Your Sapient</h3>
              <div className='w-full h-full'>
              <Scene />
              <Inventario/>
              </div>
              <div className="flex px-4 py-3 justify-end">
                <button
                  className="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-full h-10 px-4 bg-[#302447] text-white gap-2 pl-4 text-sm font-bold leading-normal tracking-[0.015em]"
                >
                  <div className="text-white" data-icon="ArrowRight" data-size="20px" data-weight="regular">
                    <svg xmlns="http://www.w3.org/2000/svg" width="20px" height="20px" fill="currentColor" viewBox="0 0 256 256">
                      <path
                        d="M221.66,133.66l-72,72a8,8,0,0,1-11.32-11.32L196.69,136H40a8,8,0,0,1,0-16H196.69L138.34,61.66a8,8,0,0,1,11.32-11.32l72,72A8,8,0,0,1,221.66,133.66Z"
                      ></path>
                    </svg>
                  </div>
                  <span className="truncate">View your Sapient</span>
                </button>
              </div>
              <h3 className="text-white text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4">Overview</h3>
              <div className="flex flex-wrap gap-3 px-4 py-3">
                <div className="flex min-w-[111px] flex-1 basis-[fit-content] flex-col gap-2 rounded-lg border border-[#443465] p-3 items-center text-center">
                  <p className="text-white tracking-light text-2xl font-bold leading-tight">4.5</p>
                  <div className="flex items-center gap-2"><p className="text-[#a593c8] text-sm font-normal leading-normal">Energy</p></div>
                </div>
                <div className="flex min-w-[111px] flex-1 basis-[fit-content] flex-col gap-2 rounded-lg border border-[#443465] p-3 items-center text-center">
                  <p className="text-white tracking-light text-2xl font-bold leading-tight">3.8</p>
                  <div className="flex items-center gap-2"><p className="text-[#a593c8] text-sm font-normal leading-normal">Mood</p></div>
                </div>
                <div className="flex min-w-[111px] flex-1 basis-[fit-content] flex-col gap-2 rounded-lg border border-[#443465] p-3 items-center text-center">
                  <p className="text-white tracking-light text-2xl font-bold leading-tight">7.5M</p>
                  <div className="flex items-center gap-2"><p className="text-[#a593c8] text-sm font-normal leading-normal">Remaining Lifetime</p></div>
                </div>
                <div className="flex min-w-[111px] flex-1 basis-[fit-content] flex-col gap-2 rounded-lg border border-[#443465] p-3 items-center text-center">
                  <p className="text-white tracking-light text-2xl font-bold leading-tight">1.5M</p>
                  <div className="flex items-center gap-2"><p className="text-[#a593c8] text-sm font-normal leading-normal">Total Lifetime</p></div>
                </div>
                <div className="flex min-w-[111px] flex-1 basis-[fit-content] flex-col gap-2 rounded-lg border border-[#443465] p-3 items-center text-center">
                  <p className="text-white tracking-light text-2xl font-bold leading-tight">5</p>
                  <div className="flex items-center gap-2"><p className="text-[#a593c8] text-sm font-normal leading-normal">Level</p></div>
                </div>
              </div>
              <div className="flex flex-col gap-3 p-4">
                <div className="flex gap-6 justify-between">
                  <p className="text-white text-base font-medium leading-normal">Energy</p>
                  <p className="text-white text-sm font-normal leading-normal">50%</p>
                </div>
                <div className="rounded bg-[#443465]"><div className="h-2 rounded bg-[#5d19e6]" style={{ width: '50%' }}></div></div>
                <p className="text-[#a593c8] text-sm font-normal leading-normal">4.5</p>
              </div>
              <div className="flex flex-col gap-3 p-4">
                <div className="flex gap-6 justify-between">
                  <p className="text-white text-base font-medium leading-normal">Mood</p>
                  <p className="text-white text-sm font-normal leading-normal">50%</p>
                </div>
                <div className="rounded bg-[#443465]"><div className="h-2 rounded bg-[#5d19e6]" style={{ width: '50%' }}></div></div>
                <p className="text-[#a593c8] text-sm font-normal leading-normal">3.8</p>
              </div>
              <div className="flex flex-col gap-3 p-4">
                <div className="flex gap-6 justify-between">
                  <p className="text-white text-base font-medium leading-normal">Remaining Lifetime</p>
                  <p className="text-white text-sm font-normal leading-normal">50%</p>
                </div>
                <div className="rounded bg-[#443465]"><div className="h-2 rounded bg-[#5d19e6]" style={{ width: '50%' }}></div></div>
                <p className="text-[#a593c8] text-sm font-normal leading-normal">7.5M</p>
              </div>
              <h3 className="text-white text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4">Actions</h3>
              <div className="flex items-center gap-4 bg-[#171122] px-4 min-h-[72px] py-2 justify-between">
                <div className="flex items-center gap-4">
                  <div className="text-white flex items-center justify-center rounded-lg bg-[#302447] shrink-0 size-12" data-icon="Gear" data-size="24px" data-weight="regular">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24px" height="24px" fill="currentColor" viewBox="0 0 256 256">
                      <path
                        d="M128,80a48,48,0,1,0,48,48A48.05,48.05,0,0,0,128,80Zm0,80a32,32,0,1,1,32-32A32,32,0,0,1,128,160Zm88-29.84q.06-2.16,0-4.32l14.92-18.64a8,8,0,0,0,1.48-7.06,107.21,107.21,0,0,0-10.88-26.25,8,8,0,0,0-6-3.93l-23.72-2.64q-1.48-1.56-3-3L186,40.54a8,8,0,0,0-3.94-6,107.71,107.71,0,0,0-26.25-10.87,8,8,0,0,0-7.06,1.49L130.16,40Q128,40,125.84,40L107.2,25.11a8,8,0,0,0-7.06-1.48A107.6,107.6,0,0,0,73.89,34.51a8,8,0,0,0-3.93,6L67.32,64.27q-1.56,1.49-3,3L40.54,70a8,8,0,0,0-6,3.94,107.71,107.71,0,0,0-10.87,26.25,8,8,0,0,0,1.49,7.06L40,125.84Q40,128,40,130.16L25.11,148.8a8,8,0,0,0-1.48,7.06,107.21,107.21,0,0,0,10.88,26.25,8,8,0,0,0,6,3.93l23.72,2.64q1.49,1.56,3,3L70,215.46a8,8,0,0,0,3.94,6,107.71,107.71,0,0,0,26.25,10.87,8,8,0,0,0,7.06-1.49L125.84,216q2.16.06,4.32,0l18.64,14.92a8,8,0,0,0,7.06,1.48,107.21,107.21,0,0,0,26.25-10.88,8,8,0,0,0,3.93-6l2.64-23.72q1.56-1.48,3-3L215.46,186a8,8,0,0,0,6-3.94,107.71,107.71,0,0,0,10.87-26.25,8,8,0,0,0-1.49-7.06Zm-16.1-6.5a73.93,73.93,0,0,1,0,8.68,8,8,0,0,0,1.74,5.48l14.19,17.73a91.57,91.57,0,0,1-6.23,15L187,173.11a8,8,0,0,0-5.1,2.64,74.11,74.11,0,0,1-6.14,6.14,8,8,0,0,0-2.64,5.1l-2.51,22.58a91.32,91.32,0,0,1-15,6.23l-17.74-14.19a8,8,0,0,0-5-1.75h-.48a73.93,73.93,0,0,1-8.68,0,8,8,0,0,0-5.48,1.74L100.45,215.8a91.57,91.57,0,0,1-15-6.23L82.89,187a8,8,0,0,0-2.64-5.1,74.11,74.11,0,0,1-6.14-6.14,8,8,0,0,0-5.1-2.64L46.43,170.6a91.32,91.32,0,0,1-6.23-15l14.19-17.74a8,8,0,0,0,1.74-5.48,73.93,73.93,0,0,1,0-8.68,8,8,0,0,0-1.74-5.48L40.2,100.45a91.57,91.57,0,0,1,6.23-15L69,82.89a8,8,0,0,0,5.1-2.64,74.11,74.11,0,0,1,6.14-6.14A8,8,0,0,0,82.89,69L85.4,46.43a91.32,91.32,0,0,1,15-6.23l17.74,14.19a8,8,0,0,0,5.48,1.74,73.93,73.93,0,0,1,8.68,0,8,8,0,0,0,5.48-1.74L155.55,40.2a91.57,91.57,0,0,1,15,6.23L173.11,69a8,8,0,0,0,2.64,5.1,74.11,74.11,0,0,1,6.14,6.14,8,8,0,0,0,5.1,2.64l22.58,2.51a91.32,91.32,0,0,1,6.23,15l-14.19,17.74A8,8,0,0,0,199.87,123.66Z"
                      ></path>
                    </svg>
                  </div>
                  <div className="flex flex-col justify-center">
                    <p className="text-white text-base font-medium leading-normal line-clamp-1">Control Panel</p>
                    <p className="text-[#a593c8] text-sm font-normal leading-normal line-clamp-2">Eat, sleep, or wake your Sapient</p>
                  </div>
                </div>
                <div className="shrink-0">
                  <div className="text-white flex size-7 items-center justify-center" data-icon="ArrowRight" data-size="24px" data-weight="regular">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24px" height="24px" fill="currentColor" viewBox="0 0 256 256">
                      <path
                        d="M221.66,133.66l-72,72a8,8,0,0,1-11.32-11.32L196.69,136H40a8,8,0,0,1,0-16H196.69L138.34,61.66a8,8,0,0,1,11.32-11.32l72,72A8,8,0,0,1,221.66,133.66Z"
                      ></path>
                    </svg>
                  </div>
                </div>
              </div>
              <div className="flex items-center gap-4 bg-[#171122] px-4 min-h-[72px] py-2 justify-between">
                <div className="flex items-center gap-4">
                  <div className="text-white flex items-center justify-center rounded-lg bg-[#302447] shrink-0 size-12" data-icon="ChatCircleDots" data-size="24px" data-weight="regular">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24px" height="24px" fill="currentColor" viewBox="0 0 256 256">
                      <path
                        d="M140,128a12,12,0,1,1-12-12A12,12,0,0,1,140,128ZM84,116a12,12,0,1,0,12,12A12,12,0,0,0,84,116Zm88,0a12,12,0,1,0,12,12A12,12,0,0,0,172,116Zm60,12A104,104,0,0,1,79.12,219.82L45.07,231.17a16,16,0,0,1-20.24-20.24l11.35-34.05A104,104,0,1,1,232,128Zm-16,0A88,88,0,1,0,51.81,172.06a8,8,0,0,1,.66,6.54L40,216,77.4,203.53a7.85,7.85,0,0,1,2.53-.42,8,8,0,0,1,4,1.08A88,88,0,0,0,216,128Z"
                      ></path>
                    </svg>
                  </div>
                  <div className="flex flex-col justify-center">
                    <p className="text-white text-base font-medium leading-normal line-clamp-1">Chat</p>
                    <p className="text-[#a593c8] text-sm font-normal leading-normal line-clamp-2">Start a conversation with your Sapient</p>
                  </div>
                </div>
                <div className="shrink-0">
                  <div className="text-white flex size-7 items-center justify-center" data-icon="ArrowRight" data-size="24px" data-weight="regular">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24px" height="24px" fill="currentColor" viewBox="0 0 256 256">
                      <path
                        d="M221.66,133.66l-72,72a8,8,0,0,1-11.32-11.32L196.69,136H40a8,8,0,0,1,0-16H196.69L138.34,61.66a8,8,0,0,1,11.32-11.32l72,72A8,8,0,0,1,221.66,133.66Z"
                      ></path>
                    </svg>
                  </div>
                </div>
              </div>
              <div className="flex items-center gap-4 bg-[#171122] px-4 min-h-[72px] py-2 justify-between">
                <div className="flex items-center gap-4">
                  <div className="text-white flex items-center justify-center rounded-lg bg-[#302447] shrink-0 size-12" data-icon="Target" data-size="24px" data-weight="regular">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24px" height="24px" fill="currentColor" viewBox="0 0 256 256">
                      <path
                        d="M221.87,83.16A104.1,104.1,0,1,1,195.67,49l22.67-22.68a8,8,0,0,1,11.32,11.32l-96,96a8,8,0,0,1-11.32-11.32l27.72-27.72a40,40,0,1,0,17.87,31.09,8,8,0,0,1,16-.9,56,56,0,1,1-22.38-41.65L184.3,60.39a87.88,87.88,0,1,0,23.13,29.67,8,8,0,0,1,14.44-6.9Z"
                      ></path>
                    </svg>
                  </div>
                  <div className="flex flex-col justify-center">
                    <p className="text-white text-base font-medium leading-normal line-clamp-1">Goals</p>
                    <p className="text-[#a593c8] text-sm font-normal leading-normal line-clamp-2">View or set your Sapient's goals</p>
                  </div>
                </div>
                <div className="shrink-0">
                  <div className="text-white flex size-7 items-center justify-center" data-icon="ArrowRight" data-size="24px" data-weight="regular">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24px" height="24px" fill="currentColor" viewBox="0 0 256 256">
                      <path
                        d="M221.66,133.66l-72,72a8,8,0,0,1-11.32-11.32L196.69,136H40a8,8,0,0,1,0-16H196.69L138.34,61.66a8,8,0,0,1,11.32-11.32l72,72A8,8,0,0,1,221.66,133.66Z"
                      ></path>
                    </svg>
                  </div>
                </div>
              </div>
              <div className="flex items-center gap-4 bg-[#171122] px-4 min-h-[72px] py-2 justify-between">
                <div className="flex items-center gap-4">
                  <div className="text-white flex items-center justify-center rounded-lg bg-[#302447] shrink-0 size-12" data-icon="List" data-size="24px" data-weight="regular">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24px" height="24px" fill="currentColor" viewBox="0 0 256 256">
                      <path
                        d="M224,128a8,8,0,0,1-8,8H40a8,8,0,0,1,0-16H216A8,8,0,0,1,224,128ZM40,72H216a8,8,0,0,0,0-16H40a8,8,0,0,0,0,16ZM216,184H40a8,8,0,0,0,0,16H216a8,8,0,0,0,0-16Z"
                      ></path>
                    </svg>
                  </div>
                  <div className="flex flex-col justify-center">
                    <p className="text-white text-base font-medium leading-normal line-clamp-1">Log</p>
                    <p className="text-[#a593c8] text-sm font-normal leading-normal line-clamp-2">View and interact with your Sapient's log</p>
                  </div>
                </div>
                <div className="shrink-0">
                  <div className="text-white flex size-7 items-center justify-center" data-icon="ArrowRight" data-size="24px" data-weight="regular">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24px" height="24px" fill="currentColor" viewBox="0 0 256 256">
                      <path
                        d="M221.66,133.66l-72,72a8,8,0,0,1-11.32-11.32L196.69,136H40a8,8,0,0,1,0-16H196.69L138.34,61.66a8,8,0,0,1,11.32-11.32l72,72A8,8,0,0,1,221.66,133.66Z"
                      ></path>
                    </svg>
                  </div>
                </div>
              </div>
              <div className="flex items-center gap-4 bg-[#171122] px-4 min-h-[72px] py-2 justify-between">
                <div className="flex items-center gap-4">
                  <div className="text-white flex items-center justify-center rounded-lg bg-[#302447] shrink-0 size-12" data-icon="TrendUp" data-size="24px" data-weight="regular">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24px" height="24px" fill="currentColor" viewBox="0 0 256 256">
                      <path
                        d="M240,56v64a8,8,0,0,1-16,0V75.31l-82.34,82.35a8,8,0,0,1-11.32,0L96,123.31,29.66,189.66a8,8,0,0,1-11.32-11.32l72-72a8,8,0,0,1,11.32,0L136,140.69,212.69,64H168a8,8,0,0,1,0-16h64A8,8,0,0,1,240,56Z"
                      ></path>
                    </svg>
                  </div>
                  <div className="flex flex-col justify-center">
                    <p className="text-white text-base font-medium leading-normal line-clamp-1">Stats</p>
                    <p className="text-[#a593c8] text-sm font-normal leading-normal line-clamp-2">View statistics about your Sapient</p>
                  </div>
                </div>
                <div className="shrink-0">
                  <div className="text-white flex size-7 items-center justify-center" data-icon="ArrowRight" data-size="24px" data-weight="regular">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24px" height="24px" fill="currentColor" viewBox="0 0 256 256">
                      <path
                        d="M221.66,133.66l-72,72a8,8,0,0,1-11.32-11.32L196.69,136H40a8,8,0,0,1,0-16H196.69L138.34,61.66a8,8,0,0,1,11.32-11.32l72,72A8,8,0,0,1,221.66,133.66Z"
                      ></path>
                    </svg>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    
    </>
  );
};

export default Interfaz;