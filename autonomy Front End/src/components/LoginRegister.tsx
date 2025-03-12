"use client";

import * as React from "react";


export default function Web3Login5() {
  return (
<div className="w-full rounded-lg border shadow-sm overflow-hidden bg-transparent border-slate-200 shadow-slate-950/5 max-w-lg mx-auto">
  <div className="h-max rounded p-8 m-0 w-full text-center">
    <h2 className="font-sans antialiased font-bold text-lg md:text-xl lg:text-2xl text-slate-800 dark:text-white mb-2 mt-4">Web3 Login Simplified</h2>
    <p className="font-sans antialiased text-base text-slate-600 max-w-lg [text-wrap:balance] mx-auto">Enjoy quick and secure access to your accounts on various Web3 platforms.</p>
  </div>
  <div className="w-full h-max rounded p-8 space-y-4">
    <div className="w-full space-y-1.5">
      <label typeof="email" className="font-sans antialiased text-sm text-slate-800 dark:text-white font-semibold">Your Email</label>
      <div className="relative w-full aria-disabled:cursor-not-allowed data-[shape=pill]:rounded-full text-base rounded-lg" data-shape="default">
        <input id="email" placeholder="name@mail.com" type="email" className="h-full w-full outline-none rounded-[inherit] leading-[inherit] focus:outline-none text-slate-800 dark:text-white placeholder:text-slate-600/60 bg-transparent ring-transparent border border-slate-200 transition-all duration-300 ease-in disabled:opacity-50 disabled:pointer-events-none data-[error=true]:border-error data-[success=true]:border-success select-none data-[shape=pill]:rounded-full py-3 px-3 ring-4 shadow-sm data-[icon-placement=start]:ps-11 data-[icon-placement=end]:pe-11 hover:border-slate-800 hover:ring-slate-800/10 focus:border-slate-800 focus:ring-slate-800/10 peer" data-error="false" data-success="false" data-shape="default" data-icon-placement="" />
      </div>
    </div>
    <button className="inline-flex items-center justify-center border align-middle select-none font-sans font-medium text-center transition-all duration-300 ease-in disabled:opacity-50 disabled:shadow-none disabled:cursor-not-allowed data-[shape=pill]:rounded-full data-[width=full]:w-full focus:shadow-none text-base rounded-md py-2.5 px-5 shadow-sm hover:shadow-lg bg-slate-800 border-slate-800 text-slate-50 hover:bg-slate-700 hover:border-slate-700" data-shape="default" data-width="full">Connect</button>
    <button className="justify-center border align-middle select-none font-sans font-medium text-center transition-all duration-300 ease-in disabled:opacity-50 disabled:shadow-none disabled:cursor-not-allowed data-[shape=pill]:rounded-full data-[width=full]:w-full focus:shadow-none text-base rounded-md py-2.5 px-5 shadow-sm hover:shadow-lg bg-transparent border-slate-200 text-slate-800 hover:bg-slate-200 flex items-center gap-3" data-shape="default" data-width="full">
      <img src="https://v3.material-tailwind.com/icon/google.svg" alt="google" className="w-5 h-5" /> Sign in with Google 
      </button>
    <button className="justify-center border align-middle select-none font-sans font-medium text-center transition-all duration-300 ease-in disabled:opacity-50 disabled:shadow-none disabled:cursor-not-allowed data-[shape=pill]:rounded-full data-[width=full]:w-full focus:shadow-none text-base rounded-md py-2.5 px-5 shadow-sm hover:shadow-lg bg-transparent border-slate-200 text-slate-800 hover:bg-slate-200 flex items-center gap-3" data-shape="default" data-width="full">
      <svg width="1.5em" strokeWidth="1.5" height="1.5em" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="currentColor" className="w-5 h-5 stroke-2">
        <path d="M19 20H5C3.89543 20 3 19.1046 3 18V9C3 7.89543 3.89543 7 5 7H19C20.1046 7 21 7.89543 21 9V18C21 19.1046 20.1046 20 19 20Z" stroke="currentColor"></path>
        <path d="M16.5 14C16.2239 14 16 13.7761 16 13.5C16 13.2239 16.2239 13 16.5 13C16.7761 13 17 13.2239 17 13.5C17 13.7761 16.7761 14 16.5 14Z" fill="currentColor" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round"></path>
        <path d="M18 7V5.60322C18 4.28916 16.7544 3.33217 15.4847 3.67075L4.48467 6.60409C3.60917 6.83756 3 7.63046 3 8.53656V9" stroke="currentColor"></path>
      </svg> Wallet Authentication </button>
  </div>
  <div className="w-full rounded px-8 pb-8 pt-0">
    <p className="font-sans antialiased text-sm text-center block mx-auto max-w-xs text-slate-600">Upon signing in, you consent to abide by our <a href="#" className="text-orange-500 dark:text-white">Terms of Service</a> &amp; <a href="#" className="text-orange-500 dark:text-white">Privacy Policy.</a>
    </p>
  </div>
</div>
  );
}