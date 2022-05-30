import type { NextPage } from "next";
import { Container, Box, Grid, useTheme } from "@mui/material";
import Default from "@/components/Templates/Default";
import AccountSettings from "@/components/Organisms/TransactionSettings/AccountSettings";
import { Alert, Link, PageTitle } from "@/components";
import { useAppSelector } from "@/hooks/store";
import { setMessage } from "@/stores/ui/uiSlice";
import { useDispatch } from "react-redux";
import SwapForm from "@/components/Organisms/swap/SwapForm";
import SwapChart from "@/components/Organisms/swap/SwapChart";
import OpenInNewRoundedIcon from "@mui/icons-material/OpenInNewRounded";

const twoColumnPageSize = {
  xs: 12,
  md: 12,
  lg: 6,
};

const Swap: NextPage = () => {
  const dispatch = useDispatch();

  const message = useAppSelector((state) => state.ui.message);

  return (
    <Default>
      <Container maxWidth="lg">
        <Box mb={25}>
          <Box textAlign="center">
            <PageTitle
              title="Swap"
              subtitle="Swap your tokens instantly and securely on Pablo."
            />
          </Box>
          <Grid mt={4} container spacing={4}>
            <Grid item {...twoColumnPageSize}>
              <SwapChart height={610}/>
            </Grid>
            <Grid item {...twoColumnPageSize}>
              <SwapForm/>
            </Grid>
          </Grid>
          {message.text && (
            <Box sx={{maxWidth: 854, margin: "auto"}}>
              <Alert
                severity={message.severity}
                alertText={message.text}
                onClose={() => dispatch(setMessage({}))}
                underlined
                centered
                action={
                  message.link ? (
                    <Link
                      href={message.link}
                      target="_blank"
                      rel="noopener"
                    >
                      <OpenInNewRoundedIcon />
                    </Link>
                  ) : undefined
                }
              />
            </Box>
          )}
          <AccountSettings />
        </Box>
      </Container>
    </Default>
  );
};

export default Swap;
