import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import { nav_bar } from "@/constants/nav.constants";
import Logo from "../logo";
import { NavUser } from "./nav-user";
import { useLocation } from "react-router-dom";
import { User } from "types/models/User";

export function AppSidebar({ userInfo }: { userInfo: User }) {
  const location = useLocation();

  return (
    <Sidebar>
      <SidebarHeader className="py-5">
        <Logo />
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup className="px-5">
          <SidebarGroupContent>
            <SidebarMenu>
              {nav_bar.map((item) => (
                <SidebarMenuItem key={item.title}>
                  <SidebarMenuButton
                    asChild
                    isActive={location.pathname === item.path}
                  >
                    <a href={item.path}>
                      <item.icon />
                      <span>{item.title}</span>
                    </a>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter className="py-3.5">
        <NavUser user={userInfo} />
      </SidebarFooter>
    </Sidebar>
  );
}
