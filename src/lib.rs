#![no_std]
use soroban_sdk::{contractimpl, contracttype, AccountId, Env, Symbol, Vec};

pub struct StockLending;

#[contracttype]
pub struct LendingPoolRecord {
  // Map depositor addresses to their share balance in the pool
  pub depositors: Map,

  // Map borrower addresses to a vector of their short sales
  pub borrowers: Map,
  
  // This pool will work for only one asset to simplify recordkeeping
  pub ticker: Symbol,
  
  // Record globally the total number of shares floating around in the pool
  pub sharesDeposited: u128,
  pub sharesLoanedOut: u128,

  // Upon closing short sales, allocate subsequent interest payments to the pool
  pub retainedEarnings: u32,
}

#[contracttype]
pub struct ShortSaleRecord {
  // This is just a sanity check, and could be removed to decrease execution cost
  pub shortSeller: AccountId,

  // How many shares did they short sell
  pub sharesBorrowed: u128,

  // Rate locked in upon trade execution, floating rates would be computationally hard
  pub interestRate: i32,
  
  // Simple way to keep track of the time that's passed since a borrow for interest calculation
  pub borrowLedgerNum: u32,

  // Initial collateral paid by borrower should be the position's market value, to avoid pool losses.
  // This is fair since you'd need the same amount of money to take the opposite side of the trade. 
  // Excess returned at cover. Borrower can deposit more into a losing trade to avoid liquidation.
  pub postedCollateralBTD: u64,

  // TBD: if liquidated at -60%, it should help to keep this value global for fast calculations.
  pub liqidationProceedsBTD: u64,
}

#[contractimpl]
impl StockLending {
  
  
  pub fn DepositShares(env: Env, sig: Signature, ticker: Symbol, amount: u64) -> Vec<Symbol> {
    assert(ticker == ticker)
    let invoker = env.invoker();
    let deposit_pool = Self::deposit_pool();
    PoolInfo.Deposits(invoker) = amount;
    Ok(())
  }
  
  fn withdraw_shares(env: Env, sig: Signature, ticker: T::AssetId, shares: T::Shares) -> u32 {
    let depositor = env.invoker();
    
	// check if the depositor has enough shares in the pool
    let deposit_pool = Env::deposit_pool(ticker);
    
	// if not, recursively close the oldest short sale until enough
	if deposit_pool < shares {
      // Get the oldest short position
      let oldest_position = Env::short_sale::iter()
          .map(|(account_id, short_sale)| (account_id, short_sale.open_time))
          .min_by(|(_, open_time1), (_, open_time2)| open_time1.cmp(open_time2));
      
	  // Check if there is any open short position
      if let Some((short_seller, _)) = oldest_position {
        // Liquidate the oldest short position
        Self::liquidate_position(short_seller, short_sale);
		Self::withdraw_shares(ticker, shares)
      } else {
        return Err(Error::<T>::NoShares);
      }
    }
	
    // decrease the deposit pool by the number of shares deposited
    Env::deposit_pool::mutate(ticker, |v| *v -= shares);
    // transfer the shares to the deposit pool
    Env::transfer(ticker, &depositor, &depositor, shares)?;
    Ok(())
}

    }
    
	// decrease the deposit pool by the number of shares withdrawn
    Env::deposit_pool::mutate(ticker, |v| *v -= shares);
    
	// transfer the shares to the depositor
    Env::transfer(ticker, &depositor, &depositor, shares)?;
    Ok(())
  }

  
  fn short_sell(sig: Signature, ticker: Symbol, shares: u64, collateral: u64) -> Symbol {
    // Ensure that the function is called by a signed account
    let invoker = env.invoker();

    // Get the deposit pool of the given asset
    let deposit_pool = Self::deposit_pool();
    let available = deposit_pool.get(ticker);

    // Check if there are enough shares in the deposit pool
    if available.is_none() || available < shares {
      return Symbol<InsufficientShares>);
    }

    // Reduce the number of shares in the deposit pool
    deposit_pool.mutate(ticker, |v| *v -= shares);

    // Reserve the cash collateral from the short seller
    Env::reserve(&invoker, collateral)?;

    // Calculate the interest rate
    let interest_rate = Self::get_interest_rate(ticker, shares);

    // Create a new struct for the short sale
    let short_sale = ShortSale {
        short_seller: invoker,
        ticker: ticker,
        shares: shares,
        interest_rate: interest_rate,
        collateral: collateral,
        proceeds: collateral,
    };
    
    // sell the stock on the SDEX at market (seperate function for limit orders?)

    // allocate the proceeds to the invoker's collateral account map

  }
  
  // create a new mapping to store the borrowed shares
  pub struct StockLendingModule<T: Trait> {
    pub deposit_pool: map hasher(blake2_128_concat) T::AssetId => u32,
    pub borrowed_shares: map hasher(blake2_128_concat) T::AccountId => BorrowedShares<T::AccountId, u32>,
  }

  
  impl<T: Trait> Module<T> for StockLendingModule {
    fn on_initialize(_n: T::BlockNumber) {
      deposit_pool = map hasher(blake2_128_concat) T::AssetId => u32::default();
    }
    fn on_finalize(_n: T::BlockNumber) {
      deposit_pool.clear();
    }
  }
  
  // check if there are sufficient shares available in the deposit pool
  fn borrow(sig: Signature, ticker: T::AssetId, amount: u32, collateral: u32) -> DispatchResult {
    let invoker = env.invoker();
    
	// check if there are sufficient shares available in the deposit pool,
	let deposit_pool = Self::deposit_pool();
    let available = deposit_pool.get(ticker);
	
	// check if the collateral is sufficient, the user must have enough balance to put up the collateral
    if available.is_none() || available < amount {
        return Err(Error::<T>::InsufficientShares);
    }
	
	// reduce the shares in the deposit pool by the amount borrowed and reserve the collateral from the borrower's account
    deposit_pool.mutate(ticker, |v| *v -= amount);
    Env::reserve(&invoker, collateral)?;
    let interest_rate = Self::get_interest_rate(ticker, amount);
    
	// Store the loan details with interest rate and other required details 
	let due_time = timestamp::get();
    let borrowed_shares = BorrowedShares {
      borrower: invoker,
      ticker: ticker,
      amount: amount,
      interest_rate: interest_rate,
      due_time: due_time,
    };
    Env::borrowed_shares::insert(&invoker, borrowed_shares);	  
    Ok(())
  }
  
  
  pub fn get_interest_rate(ticker: T::AssetId, amount: u32) -> u32 {
    let deposit_pool = Self::deposit_pool();
   
    // get the available shares in the deposit pool
    let available = deposit_pool.get(ticker); 

    // get the number of shares currently borrowed
    let demand = Self::borrowed(ticker); 

    // calculate the total supply of shares
    let supply = available + demand;

    // calculate the proportion of shares that are currently borrowed
    // relative to the total number of shares available
    let interest_rate = if supply > 0 {

    // let the interest rate scale logarithmically from 2.758 - 7.389 p.a.
    2.7182818284590452353602874713527.pow((1 - (supply / (supply + demand)).log(2)))
    } else {
      u32::from(0)
    };
    interest_rate
  }
  
  fn close_position(sig: Signature, ticker: T::AssetId) -> Symbol {
    let invoker = env.invoker();
    // check if the short seller is the borrower
    let short_sale = Env::short_sale(invoker);
    if short_sale.ticker != ticker {
        return Symbol<InvalidAssetId>;
    }
	
    // check if the short sale is still open
    if not short_sale.shares {
        return Err(Error::<T>::InvalidPosition);
    }
    
	// calculate the interest
    let interest = short_sale.collateral * short_sale.interest_rate;
    
	// calculate the profit
    let profit = short_sale.proceeds - short_sale.collateral - interest;
    
	// pay the interest to the short seller
    <pallet_balances::Module<T>>::repatriate_reserved(&short_sale.short_seller, &short_sale.short_seller, interest, BalanceStatus::Free)?;
    
	// pay the profit to the deposit pool
    Env::deposit_pool::mutate(ticker, |v| *v += profit);
    
	// remove the short sale from the mapping
    Env::short_sale::remove(invoker);
    Ok(())
  }

  // the liquidation function can be called periodically, for example, every hour
  // by Block Transfer or at any time by any outside investor who notices big moves
  fn check_liquidation() {
    // Iterate through all the short sales in the smart contract
	Env::short_sale::iter().for_each(|(short_seller, short_sale)| {
      
	  // Check the value of the short position
	  let value = Self::get_value(short_sale.ticker, short_sale.shares);
      
	  // Calculate the liquidation ratio
	  let liquidation_ratio = value / short_sale.collateral;
      
	  // If 60% margin reached, liquidate the position by buying back the shares
	  // and returning extra cash collateral to the short seller
	  if liquidation_ratio < .4 {
        Self::liquidate_position(short_seller, short_sale);
      }
    });
  }
  
  fn liquidate_position(short_seller: AccountId, short_sale: ShortSale<AccountId) {
    // Place a market buy order on the Stellar decentralized exchange using the locked up short seller funds
    let buy_order = MarketBuyOrder::<T> {
      amount: short_sale.shares,
      invoker: short_seller,
    };

    // submit a bid at MAX_PRICE and simutaneously roll over proceeds in atomic txn
    Env::market_buy(short_sale.ticker, buy_order)?;
    
	// Close the position by returning the collateral to the short seller
    Env::repatriate_reserved(&short_seller, &short_seller, short_sale.collateral, BalanceStatus::Free)?;
    
	// Pay any short interest accrued to the depositor pool
    let interest = short_sale.collateral * short_sale.interest_rate;
    Env::deposit_pool::mutate(short_sale.ticker, |v| *v += interest);
    
	// remove the short sale from the mapping
    Env::short_sale::remove(short_seller);
    Ok(())
  }
  
  fn add_margin(sig: Signature, ticker: T::AssetId, collateral: u32) -> DispatchResult {
	// Is caller a short seller borrowing this stock
	let short_seller = env.invoker();
    
	// Check if the short sale is still open
	let short_sale = Env::short_sale(short_seller);
    if short_sale.ticker != ticker {
      return Symbol<InvalidAssetId>);
    }
    if short_sale.shares == 0 {
      return Symbol<NoShares>;
    }
    
	// Reserve the additional cash collateral from the short seller
	<reserve(&short_seller, collateral)?;
    Env::short_sale::mutate(short_seller, |v| {
      v.collateral += collateral;
    });
    Ok(())
  }

  
  
  
}