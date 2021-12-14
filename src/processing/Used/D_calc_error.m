addpath(".");

close all
clear
clc
format long g

root_path = "/media/joebillingsley/Data/projects/NFV_PlacementModel_Journal";
% root_path = "D:\Research\NFV_PlacementModel_Journal";

model_folder = fullfile(root_path, 'data', 'Model');
simulation_folder = fullfile(root_path, 'results', 'Simulation');

models = [
    fullfile(model_folder, "MinEnergy"), ...
    fullfile(model_folder, "MinLatency"), ...
    fullfile(model_folder, "MinPacketLoss"), ...
    fullfile(model_folder, "Balanced")
];

simulations = [
    fullfile(simulation_folder, "SIMULATION_Low_Energy.out"), ...
    fullfile(simulation_folder, "SIMULATION_Low_Latency.out"), ...
    fullfile(simulation_folder, "SIMULATION_Low_PacketLoss.out"), ...
    fullfile(simulation_folder, "SIMULATION_Balanced.out")
];

avg_err_iter = 0;
avg_err_non_iter = 0;

avg_err_metrics_iter = zeros(1, 3);
avg_err_metrics_non_iter = zeros(1, 3);

for i = 1:length(models)
    model = models(i);
    simulation = simulations(i);
    
    iterative_path = fullfile(model, 'Iterative');
    non_iterative_path = fullfile(model, 'NoIterations');
    
    iterative = readmatrix(iterative_path);
    non_iterative = readmatrix(non_iterative_path);
    simulation = readmatrix(simulation, 'FileType', 'text');
    
    column_names = ["Latency", "Packet Loss", "Energy"];
    
    fprintf('%s \n', model);
    for j = 1:3
        col = j + 1;
        
        iter = calculate_error(simulation(:, col), iterative(:, col), 100);
        non_iter = calculate_error(simulation(:, col), non_iterative(:, col), 100);
        
        avg_err_iter = avg_err_iter + iter;
        avg_err_non_iter = avg_err_non_iter + non_iter;
        
        avg_err_metrics_iter(j) = avg_err_metrics_iter(j) + iter;
        avg_err_metrics_non_iter(j) = avg_err_metrics_non_iter(j) + non_iter;
        
        fprintf('%s: Iterative %f NonIterative %f \n', column_names(j), iter, non_iter)
    end
    fprintf('\n');
end

fprintf('Average: Iterative %f NonIterative %f\n', avg_err_iter / 9, avg_err_non_iter / 9);

disp("Metrics");
disp(avg_err_metrics_iter / 3);
disp(avg_err_metrics_non_iter / 3);

function err = calculate_error(truth, estimate, len)

err = 0;
for i = 1:len
    err = err + (truth(i) - estimate(i))^2;
end
err = sqrt(err / len);

end