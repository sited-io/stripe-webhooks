CREATE OR REPLACE FUNCTION updated_at_now()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = now(); 
   RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TABLE subscriptions (
  subscription_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  stripe_subscription_id VARCHAR NOT NULL UNIQUE,
  buyer_user_id VARCHAR,
  offer_id UUID,
  current_period_start TIMESTAMP WITH TIME ZONE,
  current_period_end TIMESTAMP WITH TIME ZONE,
  subscription_status VARCHAR,
  payed_at TIMESTAMP WITH TIME ZONE,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_shops_updated_at BEFORE UPDATE
    ON subscriptions FOR EACH ROW EXECUTE PROCEDURE 
    updated_at_now();
