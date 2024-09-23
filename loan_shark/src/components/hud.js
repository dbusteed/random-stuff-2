import { useState, useEffect, useContext } from "react"
import { DateContext, LoanContext } from "../context"

const Hud = () => {
  const [loans, setLoans] = useContext(LoanContext)
  const [income, setIncome] = useState(0)
  const [date] = useContext(DateContext)
  const days = ['Sunday','Monday','Tuesday','Wednesday','Thursday','Friday','Saturday'];

  const F0 = new Intl.NumberFormat('en-US', {
    'style': 'currency',
    'currency': 'USD',
    'minimumFractionDigits': 0,
    'maximumFractionDigits': 0,
  })

  const F2 = new Intl.NumberFormat('en-US', {
    'style': 'currency',
    'currency': 'USD',
    'minimumFractionDigits': 0,
    'maximumFractionDigits': 0,
  })

  useEffect(() => {
    setIncome(
      Object.values(loans.current).filter(x => x.type === "lender").map(x => x.p).reduce((s, a) => s + a, 0) -
      Object.values(loans.current).filter(x => x.type === "borrow").map(x => x.p).reduce((s, a) => s + a, 0)
    )
  }, [loans])

	return (
		<div className="flex flex-row justify-between px-2 py-1 mt-6" style={{border: "0.1rem solid white"}}>
      <div>
        {date.toLocaleDateString()} {days[date.getDay()]}
      </div>
      <div>
        <span className="mr-2">
          {F2.format(loans.money)}
        </span>
        <span>
          (<span className={income >= 0 ? 'lender' : 'borrow'}>{F0.format(income)}</span>/week)
        </span>
      </div>
		</div>
	)
}

export default Hud