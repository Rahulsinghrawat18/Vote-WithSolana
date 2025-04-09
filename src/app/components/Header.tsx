'use client'

import { WalletMultiButton } from '@solana/wallet-adapter-react-ui'
import Link from 'next/link'
import { useEffect, useState } from 'react'

const Header = () => {
  const [isMounted, setIsMounted] = useState(false)

  useEffect(() => {
    setIsMounted(true)
  }, [])

  return (
    <header className="p-4 border-b border-gray-300 mb-4">
      <nav className="flex justify-between items-center max-w-6xl mx-auto">
        <div className="flex justify-start items-center space-x-8">
          <Link href="/">
            <h4 className="text-black text-2xl font-extrabold">Let's Vote</h4>
          </Link>

          <div className="flex justify-start items-center space-x-2 
          bg-gray-800 text-white rounded-full px-6 py-2 t
          ext-lg font-medium">
            <Link href={'/create'}>Create</Link>
          </div>
        </div>

        {isMounted && (
          <WalletMultiButton
            style={{ backgroundColor: '#F97316', color: 'white' }}
          />
        )}
      </nav>
    </header>
  )
}

export default Header