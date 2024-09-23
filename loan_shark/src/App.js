import { useState, useEffect, useContext, createContext } from 'react'
import Hud from './components/hud'
import Loans from './components/loans'
import { LoanContext, DateContext } from './context'


function App() {
  const [date, setDate] = useState(new Date('2023-01-01 8:00'))
  const [loans, setLoans] = useState({
    avail: {},
    current: {},
    money: 500,
  })

  useEffect(() => {
    const interval = setInterval(() => {
      let newDate = new Date(date.getTime() + (1 * 24 * 60 * 60 * 1000))
      setDate(newDate)
      
      // Object.entries(loans.avail)
      //   .filter(([_lid, loan]) => loan.dueOn === newDate.getDay())
      //   .forEach(([lid, loan]) => {
      //     let copy = { ...loans.avail }
      //     delete copy[lid]
      //     setLoans({ ...loans, avali: { ...copy } })
        // })

      Object.entries(loans.current)
        .filter(([_lid, loan]) => loan.dueOn === newDate.getDay())
        .forEach(([lid, loan]) => {
          if (loan.type === 'borrow') {
            if (loan.p > loan.pv) {
              loans.money -= loan.pv
              let copy = { ...loans.current }
              delete copy[lid]
              setLoans({ ...loans, current: { ...copy } })
            }
            loans.money -= loan.p
          } else {
            if (loan.p > loan.pv) {
              loans.money += loan.pv
              let copy = { ...loans.current }
              delete copy[lid]
              setLoans({ ...loans, current: { ...copy } })
            }
            loans.money += loan.p
          }
          loan.pv -= (loan.p - (loan.pv * loan.r))
        })

      if (Math.floor(Math.random() * 7) == date.getDay()) {
        let id = Math.floor(Math.random() * 1e9)
        let pv = (Math.floor(((Math.random() * 500) / 50)) * 50) + 50
        let n = (Math.floor(((Math.random() * 100) / 10)) * 10) + 10
        let type = ['borrow', 'lender'][Math.floor(Math.random() * 2)]
        let r = (Math.floor(Math.random() * 100) + 1) / 1000
        let p =  (r * pv) / (1 - (1+r)**-n)
        let loan = {
          og: pv,
          pv: pv,
          r: r,
          p: p,
          n: n,
          type: type,
          dueOn: date.getDay(),
        }
        setLoans({...loans, avail: {...loans.avail, [id]: loan}})
      }
    }, 500);
    return () => clearInterval(interval);
  }, [date])

  return (
    <LoanContext.Provider value={[loans, setLoans]}>
      <DateContext.Provider value={[date, setDate]}>
        <div className="flex flex-row">
          <div className="flex-1"></div>
          <div style={{flex: 3}}>
            <Hud />
            <Loans />
          </div>
          <div className="flex-1"></div>
        </div>
      </DateContext.Provider>
    </LoanContext.Provider>
  )
}

export default App
