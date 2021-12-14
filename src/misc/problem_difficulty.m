
prob_feasible = zeros(7, 100);

i = 1;
for k = [4, 8, 12, 16, 20, 24, 28, 32]
    
    N = ((k^3) / 4) * 3;
    
    j = 1;
    for perc = 0 : 0.01 : 1
        v = round(perc * N) + 1;
        
        p_one_placed = (1 - (1 - (1/v))^N);
        p_one_not_placed = 1 - p_one_placed ^ v;
        
        prob_feasible(i, j) = 1 - p_one_not_placed;
        
        j = j + 1;
    end
    
    i = i + 1;
end

prob_feasible = [0:0.01:1.0; prob_feasible];
prob_feasible = prob_feasible';


root_path = "/media/joebillingsley/Data/projects/NFV_PlacementModel_Journal";
% root_path = "D:\Research\NFV_PlacementModel_Journal";

writematrix(prob_feasible, fullfile(root_path, 'processed', 'prob_feasible.csv'));