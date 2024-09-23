import { useContext } from "react"
import { LoanContext, DateContext } from "../context"

const Loans = () => {
  const [loans, setLoans] = useContext(LoanContext)
  const [date] = useContext(DateContext)
  
  const F0 = new Intl.NumberFormat('en-US', {
    'style': 'currency',
    'currency': 'USD',
    'minimumFractionDigits': 0,
    'maximumFractionDigits': 0,
  })

  const processLoan = (lid) => {
    if (loans.avail[lid].type === 'lender') {
      if (loans.avail[lid].og > loans.money) return
      let loan = loans.avail[lid]
      loans.money -= loan.og
      loan.dueOn = date.getDay()
      let copy = {...loans.avail}
      delete copy[lid]
      setLoans({...loans, avail: {...copy}, current: {...loans.current, [lid]: loan}})
    } else {
      let loan = loans.avail[lid]
      loans.money += loan.og
      loan.dueOn = date.getDay()
      let copy = {...loans.avail}
      delete copy[lid]
      setLoans({...loans, avail: {...copy}, current: {...loans.current, [lid]: loan}})
    }
  }

  return (
    <div className="flex flex-row justify-around mt-4">
      <div className="flex-1 flex flex-col items-center">
        <span className="text-xl underline underline-offset-4">loan market</span>
        {
          Object.entries(loans.avail).map(([lid, loan]) => (
            <div className={`loan-card loan-card-${loan.type}`} onClick={() => processLoan(lid)}>
              <p>{loan.type.toUpperCase()} {F0.format(loan.pv)} @ {(loan.r * 100).toFixed(2)}%, pay <span className={loan.type}>{F0.format(loan.p)}</span> for {loan.n} wks</p>
            </div>
          ))
        }
      </div>
      <div className="flex-1 flex flex-col items-center">
      <span className="text-xl underline underline-offset-4">my loans</span>
        {
          Object.entries(loans.current).map(([lid, loan]) => (
            <div className={`loan-card loan-card-${loan.type}`} onClick={() => processLoan(lid)}>
              <p>{loan.type.toUpperCase()} {F0.format(loan.pv)} @ {(loan.r * 100).toFixed(2)}%, pay <span className={loan.type}>{F0.format(loan.p)}</span> for {loan.n} wks</p>
            </div>
          ))
        }
      </div>
    </div>
  )
}

export default Loans